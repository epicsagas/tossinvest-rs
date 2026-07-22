use crate::domain::models::{Account, HoldingsOverview};
use crate::error::SdkError;
use async_trait::async_trait;

/// 계좌·잔고 포트.
///
/// `get_holdings` 는 계좌(`AccountSeq`) 헤더가 필요하므로
/// [`crate::adapter::http_client::HttpClient::with_account`] 로 기본 계좌를 지정해야 합니다.
#[async_trait]
pub trait AccountPort {
    /// 보유 계좌 목록 (`GET /api/v1/accounts`). 각 계좌의 `accountSeq` 를 확인하세요.
    async fn get_accounts(&self) -> Result<Vec<Account>, SdkError>;
    /// 보유 종목 종합 (`GET /api/v1/holdings`). `symbol` 로 특정 종목 필터.
    async fn get_holdings(&self, symbol: Option<&str>) -> Result<HoldingsOverview, SdkError>;
}
