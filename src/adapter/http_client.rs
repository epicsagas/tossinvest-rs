//! HTTP 클라이언트 — 모든 포트 trait 의 구체 구현 진입점.
//!
//! OAuth2 액세스 토큰을 자동 발급·주입하고, 계좌 기반 엔드포인트에는
//! `AccountSeq` 헤더를 추가하며, `{"result": T}` envelope 을 풀어 `T` 를 반환합니다.

use std::time::Duration;

use reqwest::{Method, RequestBuilder};

use crate::adapter::token::TokenStore;
use crate::domain::models::ApiResponse;
use crate::error::SdkError;

const DEFAULT_BASE_URL: &str = "https://openapi.tossinvest.com";

/// 토스증권 Open API 클라이언트. 모든 포트 trait (`MarketDataPort` 등) 을 구현합니다.
///
/// ```no_run
/// use tossinvest_rs::v1::{HttpClient, MarketDataPort};
/// # async fn run() -> Result<(), tossinvest_rs::v1::SdkError> {
/// let client = HttpClient::new("CLIENT_ID", "CLIENT_SECRET")?;
/// let price = client.get_prices(&["005930"]).await?;
/// # Ok(()) }
/// ```
pub struct HttpClient {
    http: reqwest::Client,
    tokens: TokenStore,
    base_url: String,
    default_account_seq: Option<i64>,
}

impl HttpClient {
    /// 클라이언트 자격증명(`client_id`, `client_secret`) 으로 클라이언트를 초기화합니다.
    pub fn new(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
    ) -> Result<Self, SdkError> {
        Self::build(client_id, client_secret, DEFAULT_BASE_URL.to_string())
    }

    fn build(
        client_id: impl Into<String>,
        client_secret: impl Into<String>,
        base_url: String,
    ) -> Result<Self, SdkError> {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;
        let tokens = TokenStore::new(
            http.clone(),
            base_url.clone(),
            client_id.into(),
            client_secret.into(),
        );
        Ok(Self {
            http,
            tokens,
            base_url,
            default_account_seq: None,
        })
    }

    /// API 베이스 URL을 덮어씁니다. mock 서버(테스트)나 사설 엔드포인트에 연결할 때 사용합니다.
    pub fn with_base_url(mut self, base_url: impl Into<String>) -> Self {
        let base_url = base_url.into();
        self.tokens.set_base_url(base_url.clone());
        self.base_url = base_url;
        self
    }

    /// 계좌 기반 엔드포인트(잔고·주문·조건부주문 등) 가 사용할 기본 계좌(`accountSeq`) 를 지정합니다.
    /// `GET /accounts` 로 얻은 `accountSeq` 값을 전달하세요.
    pub fn with_account(mut self, account_seq: i64) -> Self {
        self.default_account_seq = Some(account_seq);
        self
    }

    /// 토큰 저장소에 대한 직접 접근 (`AuthPort::issue_token` 구현용).
    pub(crate) fn tokens(&self) -> &TokenStore {
        &self.tokens
    }

    /// 계좌 기반 엔드포인트에 필요한 `AccountSeq` 헤더값을 결정합니다.
    fn account_seq(&self, needs_account: bool) -> Result<Option<i64>, SdkError> {
        if needs_account {
            self.default_account_seq
                .ok_or_else(|| SdkError::Unknown("account_seq not set; call with_account()".into()))
                .map(Some)
        } else {
            Ok(None)
        }
    }

    async fn authed(&self, method: Method, path: &str) -> Result<RequestBuilder, SdkError> {
        let token = self.tokens.access_token().await?;
        Ok(self
            .http
            .request(method, format!("{}{}", self.base_url, path))
            .bearer_auth(token))
    }

    fn with_account_header(
        &self,
        mut req: RequestBuilder,
        needs_account: bool,
    ) -> Result<RequestBuilder, SdkError> {
        if let Some(seq) = self.account_seq(needs_account)? {
            req = req.header("AccountSeq", seq.to_string());
        }
        Ok(req)
    }

    /// 본문이 있는 요청을 보내고 성공 응답 본문을 `ApiResponse<R>` 로 풀어 `R` 을 반환합니다.
    async fn parse_result<R: serde::de::DeserializeOwned>(
        &self,
        req: RequestBuilder,
    ) -> Result<R, SdkError> {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_error_response(status, &body));
        }
        let parsed: ApiResponse<R> = resp.json().await?;
        Ok(parsed.result)
    }

    /// 본문이 없는 요청(204 No Content 등) 의 상태 코드만 검증합니다.
    async fn check_status(&self, req: RequestBuilder) -> Result<(), SdkError> {
        let resp = req.send().await?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(SdkError::from_error_response(status, &body));
        }
        Ok(())
    }

    /// `GET` 요청. `query` 는 `(key, value)` 쌍의 목록.
    pub(crate) async fn get<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&'static str, String)],
        needs_account: bool,
    ) -> Result<R, SdkError> {
        let req = self.authed(Method::GET, path).await?.query(query);
        let req = self.with_account_header(req, needs_account)?;
        self.parse_result(req).await
    }

    /// 본문이 있는 `POST` 요청.
    pub(crate) async fn post<B: serde::Serialize, R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
        needs_account: bool,
    ) -> Result<R, SdkError> {
        let req = self.authed(Method::POST, path).await?.json(body);
        let req = self.with_account_header(req, needs_account)?;
        self.parse_result(req).await
    }

    /// 본문이 없고 응답 본문이 있는 `POST` 요청 (예: 주문 취소).
    pub(crate) async fn post_no_body<R: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        needs_account: bool,
    ) -> Result<R, SdkError> {
        let req = self.authed(Method::POST, path).await?;
        let req = self.with_account_header(req, needs_account)?;
        self.parse_result(req).await
    }

    /// 본문·응답이 없는 `DELETE` 요청.
    pub(crate) async fn delete_void(
        &self,
        path: &str,
        needs_account: bool,
    ) -> Result<(), SdkError> {
        let req = self.authed(Method::DELETE, path).await?;
        let req = self.with_account_header(req, needs_account)?;
        self.check_status(req).await
    }
}
