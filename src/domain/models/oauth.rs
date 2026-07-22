//! OAuth2 액세스 토큰 응답 모델.

use serde::{Deserialize, Serialize};

/// `POST /oauth2/token` 발급 성공 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuth2TokenResponse {
    /// JWT 액세스 토큰. `Authorization: Bearer` 헤더에 사용.
    pub access_token: String,
    /// 토큰 타입. 항상 `Bearer`.
    pub token_type: String,
    /// 토큰 만료까지 남은 초.
    pub expires_in: i64,
}
