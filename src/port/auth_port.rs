use crate::domain::models::OAuth2TokenResponse;
use crate::error::SdkError;
use async_trait::async_trait;

/// OAuth2 인증 포트. 수동 토큰 발급을 노출합니다.
///
/// 일반적인 API 호출에는 토큰이 자동 주입되므로 이 포트를 직접 쓸 필요가 없습니다.
/// 토큰을 명시적으로 미리 발급받거나 만료 정보를 확인하고 싶을 때 사용합니다.
#[async_trait]
pub trait AuthPort {
    /// 새 액세스 토큰을 발급받아 캐시에 저장합니다.
    async fn issue_token(&self) -> Result<OAuth2TokenResponse, SdkError>;
}
