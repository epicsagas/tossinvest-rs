//! 거래대금·거래량·등락률 순위 모델.

use serde::{Deserialize, Serialize};

use super::common::Currency;

/// 순위 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingResponse {
    /// 순위 집계 시각.
    #[serde(default)]
    pub rankedAt: Option<String>,
    pub rankings: Vec<RankingItem>,
}

/// 순위 항목.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingItem {
    /// 순위 (1부터 시작).
    pub rank: i64,
    pub symbol: String,
    pub currency: Currency,
    pub price: RankingPrice,
    pub tradingVolume: String,
    pub tradingAmount: String,
}

/// 순위의 가격 정보.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankingPrice {
    pub lastPrice: String,
    pub basePrice: String,
    #[serde(default)]
    pub changeRate: Option<String>,
}
