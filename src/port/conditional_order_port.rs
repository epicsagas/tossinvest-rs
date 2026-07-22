use crate::domain::models::{
    ConditionalOrderCreateRequest, ConditionalOrderCreateResponse, ConditionalOrderDetailResponse,
    ConditionalOrderModifyRequest, ConditionalOrderResponse, OrderListStatus,
    PaginatedConditionalOrderResponse,
};
use crate::error::SdkError;
use async_trait::async_trait;

/// 조건부(자동)주문 포트. 모든 메서드는 계좌(`AccountSeq`) 헤더가 필요합니다.
#[async_trait]
pub trait ConditionalOrderPort {
    /// 조건주문 생성 (`POST /api/v1/conditional-orders`).
    async fn create_conditional_order(
        &self,
        request: ConditionalOrderCreateRequest,
    ) -> Result<ConditionalOrderCreateResponse, SdkError>;
    /// 조건주문 목록 조회 (`GET /api/v1/conditional-orders`).
    async fn get_conditional_orders(
        &self,
        status: OrderListStatus,
        symbol: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedConditionalOrderResponse, SdkError>;
    /// 조건주문 단건 조회 (`GET /api/v1/conditional-orders/{conditionalOrderId}`).
    async fn get_conditional_order(
        &self,
        conditional_order_id: &str,
    ) -> Result<ConditionalOrderDetailResponse, SdkError>;
    /// 조건주문 취소 (`DELETE /api/v1/conditional-orders/{conditionalOrderId}`). 204 No Content.
    async fn cancel_conditional_order(&self, conditional_order_id: &str) -> Result<(), SdkError>;
    /// 조건주문 수정 (`POST /api/v1/conditional-orders/{conditionalOrderId}/modify`).
    async fn modify_conditional_order(
        &self,
        conditional_order_id: &str,
        request: ConditionalOrderModifyRequest,
    ) -> Result<ConditionalOrderResponse, SdkError>;
}
