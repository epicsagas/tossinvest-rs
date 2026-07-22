//! 계좌·잔고 모델.

use serde::{Deserialize, Serialize};

use super::common::{AccountType, Currency, MarketCountry, Price};

/// 계좌.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// 계좌번호.
    pub accountNo: String,
    /// API 요청 시 사용할 계좌 식별자 (`AccountSeq` 헤더 값).
    pub accountSeq: i64,
    pub accountType: AccountType,
}

/// 보유 종목 종합 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsOverview {
    pub totalPurchaseAmount: Price,
    pub marketValue: OverviewMarketValue,
    pub profitLoss: OverviewProfitLoss,
    pub dailyProfitLoss: OverviewDailyProfitLoss,
    pub items: Vec<HoldingsItem>,
}

/// 평가금액 요약.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewMarketValue {
    pub amount: Price,
    pub amountAfterCost: Price,
}

/// 손익 요약.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewProfitLoss {
    pub amount: Price,
    pub amountAfterCost: Price,
    pub rate: String,
    pub rateAfterCost: String,
}

/// 일간 손익 요약.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverviewDailyProfitLoss {
    pub amount: Price,
    pub rate: String,
}

/// 보유 종목 항목.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoldingsItem {
    pub symbol: String,
    pub name: String,
    pub marketCountry: MarketCountry,
    pub currency: Currency,
    pub quantity: String,
    pub lastPrice: String,
    pub averagePurchasePrice: String,
    pub marketValue: MarketValue,
    pub profitLoss: ProfitLoss,
    pub dailyProfitLoss: DailyProfitLoss,
    pub cost: Cost,
}

/// 종목별 평가금액.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketValue {
    pub purchaseAmount: String,
    pub amount: String,
    pub amountAfterCost: String,
}

/// 종목별 손익.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitLoss {
    pub amount: String,
    pub amountAfterCost: String,
    pub rate: String,
    pub rateAfterCost: String,
}

/// 종목별 일간 손익.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyProfitLoss {
    pub amount: String,
    pub rate: String,
}

/// 수수료·세금.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub commission: String,
    #[serde(default)]
    pub tax: Option<String>,
}
