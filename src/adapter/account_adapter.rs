use crate::adapter::http_client::HttpClient;
use crate::adapter::push_opt;
use crate::domain::models::{Account, HoldingsOverview};
use crate::error::SdkError;
use crate::port::AccountPort;
use async_trait::async_trait;

#[async_trait]
impl AccountPort for HttpClient {
    async fn get_accounts(&self) -> Result<Vec<Account>, SdkError> {
        self.get("/api/v1/accounts", &[], false).await
    }

    async fn get_holdings(&self, symbol: Option<&str>) -> Result<HoldingsOverview, SdkError> {
        let mut query: Vec<(&'static str, String)> = Vec::new();
        push_opt(&mut query, "symbol", symbol);
        self.get("/api/v1/holdings", &query, true).await
    }
}
