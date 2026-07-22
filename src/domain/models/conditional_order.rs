//! 조건부(자동)주문 모델.

use serde::{Deserialize, Serialize};

use super::common::{MarketCountry, OrderSide, OrderType};

/// 조건주문 타입.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionalOrderType {
    /// 단일 조건 감시.
    Single,
    /// One-Cancels-the-Other: 두 조건 동시 감시, 하나 충족 시 나머지 자동 취소.
    Oco,
    /// One-Triggers-the-Other: `first` 체결 후 `second` 감시 시작.
    Oto,
}

/// 조건주문(그룹) 상태.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionalOrderStatus {
    Watching,
    Paused,
    Ordering,
    Ordered,
    Completed,
    Expired,
}

/// 조건(leg) 상태.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionalOrderLegStatus {
    Watching,
    Holding,
    Paused,
    Ordering,
    Ordered,
    Completed,
    Expired,
    Canceled,
}

/// 조건 트리거 유형.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ConditionalOrderLegType {
    /// 지정가 도달(스탑).
    Stop,
    /// 목표 수익률 도달.
    ProfitRate,
}

/// 감시 조건(요청용). "이 가격에 닿으면 매매".
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionRequest {
    pub order_side: OrderSide,
    pub trigger_price: String,
    pub order_price: String,
}

impl ConditionRequest {
    pub fn new(
        order_side: OrderSide,
        trigger_price: impl Into<String>,
        order_price: impl Into<String>,
    ) -> Self {
        Self {
            order_side,
            trigger_price: trigger_price.into(),
            order_price: order_price.into(),
        }
    }
}

/// 감시 조건(응답용).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOrderCondition {
    #[serde(rename = "type")]
    pub leg_type: ConditionalOrderLegType,
    pub status: ConditionalOrderLegStatus,
    #[serde(default)]
    pub triggerPrice: Option<String>,
    #[serde(default)]
    pub targetProfitRate: Option<String>,
    #[serde(default)]
    pub orderPrice: Option<String>,
    #[serde(default)]
    pub triggeredOrderId: Option<String>,
}

/// 조건주문 생성 요청.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalOrderCreateRequest {
    pub symbol: String,
    #[serde(rename = "type")]
    pub conditional_type: ConditionalOrderType,
    pub quantity: String,
    pub order_type: OrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    pub expire_date: String,
    pub first: ConditionRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<ConditionRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_high_value_order: Option<bool>,
}

impl ConditionalOrderCreateRequest {
    pub fn new(
        symbol: impl Into<String>,
        conditional_type: ConditionalOrderType,
        quantity: impl Into<String>,
        order_type: OrderType,
        expire_date: impl Into<String>,
        first: ConditionRequest,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            conditional_type,
            quantity: quantity.into(),
            order_type,
            client_order_id: None,
            expire_date: expire_date.into(),
            first,
            second: None,
            confirm_high_value_order: None,
        }
    }

    /// OCO/OTO 의 두 번째 조건.
    pub fn second(mut self, second: ConditionRequest) -> Self {
        self.second = Some(second);
        self
    }

    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    pub fn confirm_high_value_order(mut self, confirm: bool) -> Self {
        self.confirm_high_value_order = Some(confirm);
        self
    }
}

/// 조건주문 수정 요청. 조건주문 전체를 재설정하므로 유지할 조건도 함께 전달합니다.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConditionalOrderModifyRequest {
    #[serde(rename = "type")]
    pub conditional_type: ConditionalOrderType,
    pub quantity: String,
    pub order_type: OrderType,
    pub expire_date: String,
    pub first: ConditionRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second: Option<ConditionRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_high_value_order: Option<bool>,
}

impl ConditionalOrderModifyRequest {
    pub fn new(
        conditional_type: ConditionalOrderType,
        quantity: impl Into<String>,
        order_type: OrderType,
        expire_date: impl Into<String>,
        first: ConditionRequest,
    ) -> Self {
        Self {
            conditional_type,
            quantity: quantity.into(),
            order_type,
            expire_date: expire_date.into(),
            first,
            second: None,
            confirm_high_value_order: None,
        }
    }

    pub fn second(mut self, second: ConditionRequest) -> Self {
        self.second = Some(second);
        self
    }

    pub fn confirm_high_value_order(mut self, confirm: bool) -> Self {
        self.confirm_high_value_order = Some(confirm);
        self
    }
}

/// 조건주문 생성 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOrderCreateResponse {
    pub conditionalOrderId: String,
    #[serde(default)]
    pub clientOrderId: Option<String>,
}

/// 조건주문 수정·취소 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOrderResponse {
    pub conditionalOrderId: String,
}

/// 조건주문 상세 (목록 항목 / 상세 공용).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConditionalOrderDetailResponse {
    pub conditionalOrderId: String,
    #[serde(rename = "type")]
    pub conditional_type: ConditionalOrderType,
    pub status: ConditionalOrderStatus,
    pub symbol: String,
    pub market: MarketCountry,
    pub quantity: String,
    pub orderType: OrderType,
    pub expireDate: String,
    pub first: ConditionalOrderCondition,
    #[serde(default)]
    pub second: Option<ConditionalOrderCondition>,
    pub createdAt: String,
}

/// 조건주문 페이지 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedConditionalOrderResponse {
    pub conditionalOrders: Vec<ConditionalOrderDetailResponse>,
    #[serde(default)]
    pub nextCursor: Option<String>,
    pub hasNext: bool,
}
