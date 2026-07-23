//! OAuth2 Client Credentials 토큰 관리.
//!
//! `POST /oauth2/token` 으로 발급받은 액세스 토큰을 만료 시간 기반으로 캐싱하고,
//! 만료(여유분 포함) 시점에 자동 갱신합니다.

use std::sync::Mutex;
use std::time::{Duration, Instant};

use crate::domain::models::OAuth2TokenResponse;
use crate::error::SdkError;

/// 만료 이전에 미리 갱신하는 여유 시간(초). 시계 드리프트·네트워크 지연 대비.
const EARLY_REFRESH_SECS: u64 = 60;

pub(crate) struct TokenStore {
    http: reqwest::Client,
    base_url: String,
    client_id: String,
    client_secret: String,
    token: Mutex<Option<CachedToken>>,
}

#[derive(Clone)]
struct CachedToken {
    access_token: String,
    /// 이 시점 이후면 만료로 간주(이미 갱신 여유분을 뺀 값).
    expires_at: Instant,
}

impl TokenStore {
    pub(crate) fn new(
        http: reqwest::Client,
        base_url: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            http,
            base_url,
            client_id,
            client_secret,
            token: Mutex::new(None),
        }
    }

    pub(crate) fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    /// 유효한 액세스 토큰을 반환합니다. 캐시가 만료/부재하면 새로 발급합니다.
    pub(crate) async fn access_token(&self) -> Result<String, SdkError> {
        if let Some(token) = self.cached() {
            return Ok(token);
        }
        self.refresh().await
    }

    /// 토큰 발급을 강제 수행합니다 (`AuthPort::issue_token`).
    pub(crate) async fn issue(&self) -> Result<OAuth2TokenResponse, SdkError> {
        let resp = self.fetch().await?;
        self.store(&resp);
        Ok(resp)
    }

    fn cached(&self) -> Option<String> {
        let guard = self.token.lock().ok()?;
        guard
            .as_ref()
            .filter(|t| t.expires_at > Instant::now())
            .map(|t| t.access_token.clone())
    }

    async fn refresh(&self) -> Result<String, SdkError> {
        // 캐시 확인은 `std::sync::Mutex` 로 짧게 잡고, 네트워크 발급은 락 밖에서 수행합니다.
        let resp = self.fetch().await?;
        let token = resp.access_token.clone();
        self.store(&resp);
        Ok(token)
    }

    async fn fetch(&self) -> Result<OAuth2TokenResponse, SdkError> {
        let resp = self
            .http
            .post(format!("{}/oauth2/token", self.base_url))
            .form(&[
                ("grant_type", "client_credentials"),
                ("client_id", self.client_id.as_str()),
                ("client_secret", self.client_secret.as_str()),
            ])
            .send()
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await?;
            return Err(SdkError::from_error_response(status, &body));
        }
        resp.json::<OAuth2TokenResponse>()
            .await
            .map_err(SdkError::from)
    }

    fn store(&self, resp: &OAuth2TokenResponse) {
        let ttl = (resp.expires_in.max(0) as u64).saturating_sub(EARLY_REFRESH_SECS);
        let cached = CachedToken {
            access_token: resp.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(ttl.max(1)),
        };
        if let Ok(mut guard) = self.token.lock() {
            *guard = Some(cached);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_and_read_cached_token() {
        let store = TokenStore::new(
            reqwest::Client::new(),
            "x".into(),
            "id".into(),
            "secret".into(),
        );
        store.store(&OAuth2TokenResponse {
            access_token: "tok".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
        });
        assert_eq!(store.cached().as_deref(), Some("tok"));
    }

    /// `expires_in` 이 갱신 여유분(60s) 미만이어도 `ttl.max(1)` 로 최소 1초는 캐시(0-duration·
    /// 즉시 만료 방지). 만료 이후 `cached()` 가 `None` 이 되는 분기는 `Instant::now()` 의존이라
    /// 결정론적 단위 테스트가 불가 — e2e에서는 토큰 재사용(`expect(1)`)으로 검증합니다.
    #[test]
    fn store_with_short_expiry_still_cached_briefly() {
        let store = TokenStore::new(
            reqwest::Client::new(),
            "x".into(),
            "id".into(),
            "secret".into(),
        );
        store.store(&OAuth2TokenResponse {
            access_token: "short".into(),
            token_type: "Bearer".into(),
            expires_in: 0,
        });
        assert_eq!(store.cached().as_deref(), Some("short"));
    }

    #[test]
    fn store_overwrites_previous_token() {
        let store = TokenStore::new(
            reqwest::Client::new(),
            "x".into(),
            "id".into(),
            "secret".into(),
        );
        store.store(&OAuth2TokenResponse {
            access_token: "first".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
        });
        store.store(&OAuth2TokenResponse {
            access_token: "second".into(),
            token_type: "Bearer".into(),
            expires_in: 3600,
        });
        assert_eq!(store.cached().as_deref(), Some("second"));
    }
}
