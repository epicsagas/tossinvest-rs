use crate::domain::models::{
    BuyingPowerResponse, Commission, Currency, Order, OrderCreateRequest, OrderListStatus,
    OrderModifyRequest, OrderOperationResponse, OrderResponse, PaginatedOrderResponse,
    SellableQuantityResponse,
};
use crate::error::SdkError;
use async_trait::async_trait;

/// 주문·매수가능금액 포트. 모든 메서드는 계좌(`AccountSeq`) 헤더가 필요합니다.
#[async_trait]
pub trait TradingPort {
    /// 주문 목록 조회 (`GET /api/v1/orders`).
    async fn get_orders(
        &self,
        status: OrderListStatus,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedOrderResponse, SdkError>;
    /// 주문 생성 (`POST /api/v1/orders`).
    async fn create_order(&self, request: OrderCreateRequest) -> Result<OrderResponse, SdkError>;
    /// 주문 단건 조회 (`GET /api/v1/orders/{orderId}`).
    async fn get_order(&self, order_id: &str) -> Result<Order, SdkError>;
    /// 주문 정정 (`POST /api/v1/orders/{orderId}/modify`).
    async fn modify_order(
        &self,
        order_id: &str,
        request: OrderModifyRequest,
    ) -> Result<OrderOperationResponse, SdkError>;
    /// 주문 취소 (`POST /api/v1/orders/{orderId}/cancel`).
    async fn cancel_order(&self, order_id: &str) -> Result<OrderOperationResponse, SdkError>;
    /// 매수 가능 금액 조회 (`GET /api/v1/buying-power`).
    async fn get_buying_power(&self, currency: Currency) -> Result<BuyingPowerResponse, SdkError>;
    /// 매도 가능 수량 조회 (`GET /api/v1/sellable-quantity`).
    async fn get_sellable_quantity(
        &self,
        symbol: &str,
    ) -> Result<SellableQuantityResponse, SdkError>;
    /// 수수료율 조회 (`GET /api/v1/commissions`).
    async fn get_commissions(&self) -> Result<Vec<Commission>, SdkError>;
}
