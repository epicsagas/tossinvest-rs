use crate::adapter::http_client::HttpClient;
use crate::domain::models::OAuth2TokenResponse;
use crate::error::SdkError;
use crate::port::AuthPort;
use async_trait::async_trait;

#[async_trait]
impl AuthPort for HttpClient {
    async fn issue_token(&self) -> Result<OAuth2TokenResponse, SdkError> {
        self.tokens().issue().await
    }
}
