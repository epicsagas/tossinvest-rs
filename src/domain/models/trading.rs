//! 매수가능금액·매도가능수량·수수료·주문 목록(페이지) 모델.

use serde::{Deserialize, Serialize};

use super::common::Currency;
use super::order::Order;

/// 매수 가능 금액 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyingPowerResponse {
    pub currency: Currency,
    pub cashBuyingPower: String,
}

/// 수수료율.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Commission {
    pub marketCountry: super::common::MarketCountry,
    pub commissionRate: String,
    #[serde(default)]
    pub startDate: Option<String>,
    #[serde(default)]
    pub endDate: Option<String>,
}

/// 매도 가능 수량 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SellableQuantityResponse {
    pub sellableQuantity: String,
}

/// 주문 목록 페이지 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedOrderResponse {
    pub orders: Vec<Order>,
    #[serde(default)]
    pub nextCursor: Option<String>,
    pub hasNext: bool,
}
