//! 주문 요청·응답 모델.

use serde::{Deserialize, Serialize};

use super::common::{Currency, OrderSide, OrderStatus, OrderType, TimeInForce};

/// 주문 정보.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub orderId: String,
    pub symbol: String,
    pub side: OrderSide,
    pub orderType: OrderType,
    pub timeInForce: TimeInForce,
    pub status: OrderStatus,
    #[serde(default)]
    pub price: Option<String>,
    pub quantity: String,
    #[serde(default)]
    pub orderAmount: Option<String>,
    pub currency: Currency,
    pub orderedAt: String,
    #[serde(default)]
    pub canceledAt: Option<String>,
    pub execution: OrderExecution,
}

/// 주문 체결 결과.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderExecution {
    pub filledQuantity: String,
    #[serde(default)]
    pub averageFilledPrice: Option<String>,
    #[serde(default)]
    pub filledAmount: Option<String>,
    #[serde(default)]
    pub commission: Option<String>,
    #[serde(default)]
    pub tax: Option<String>,
    #[serde(default)]
    pub filledAt: Option<String>,
    #[serde(default)]
    pub settlementDate: Option<String>,
}

/// 주문 생성 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderResponse {
    pub orderId: String,
    #[serde(default)]
    pub clientOrderId: Option<String>,
}

/// 주문 정정·취소 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderOperationResponse {
    pub orderId: String,
}

/// 주문 생성 요청.
///
/// 수량 기반(`quantity`) 과 금액 기반(`orderAmount`) 두 형태를 단일 구조체로 표현합니다.
/// [`OrderCreateRequest::quantity_based`] / [`OrderCreateRequest::amount_based`]
/// 생성자로 만들고, 빌더 메서드로 선택 필드를 채웁니다. `None` 필드는 직렬화에서 제외됩니다.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderCreateRequest {
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    /// 수량 기반 주문의 수량.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<String>,
    /// 지정가 주문 가격 (수량 기반 + `LIMIT`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// 체결 조건 (수량 기반).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// 금액 기반 주문의 주문 금액.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_amount: Option<String>,
    /// 클라이언트 주문 식별자 (멱등키).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// 1억원 이상 고가 주문 동의 여부.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirm_high_value_order: Option<bool>,
}

impl OrderCreateRequest {
    /// 수량 기반 주문. `order_type` 이 `LIMIT` 이면 [`Self::price`] 로 가격을 지정합니다.
    pub fn quantity_based(
        symbol: impl Into<String>,
        side: OrderSide,
        order_type: OrderType,
        quantity: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type,
            quantity: Some(quantity.into()),
            price: None,
            time_in_force: None,
            order_amount: None,
            client_order_id: None,
            confirm_high_value_order: None,
        }
    }

    /// 금액 기반 주문 (시장가). `orderType` 은 항상 `MARKET` 입니다.
    pub fn amount_based(
        symbol: impl Into<String>,
        side: OrderSide,
        order_amount: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type: OrderType::Market,
            quantity: None,
            price: None,
            time_in_force: None,
            order_amount: Some(order_amount.into()),
            client_order_id: None,
            confirm_high_value_order: None,
        }
    }

    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
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

/// 주문 정정 요청.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderModifyRequest {
    pub order_type: OrderType,
    pub quantity: String,
    pub price: String,
    pub confirm_high_value_order: bool,
}

impl OrderModifyRequest {
    pub fn new(
        order_type: OrderType,
        quantity: impl Into<String>,
        price: impl Into<String>,
        confirm_high_value_order: bool,
    ) -> Self {
        Self {
            order_type,
            quantity: quantity.into(),
            price: price.into(),
            confirm_high_value_order,
        }
    }
}
