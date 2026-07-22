use crate::adapter::http_client::HttpClient;
use crate::adapter::push_opt;
use crate::domain::models::{
    ConditionalOrderCreateRequest, ConditionalOrderCreateResponse, ConditionalOrderDetailResponse,
    ConditionalOrderModifyRequest, ConditionalOrderResponse, OrderListStatus,
    PaginatedConditionalOrderResponse,
};
use crate::error::SdkError;
use crate::port::ConditionalOrderPort;
use async_trait::async_trait;

#[async_trait]
impl ConditionalOrderPort for HttpClient {
    async fn create_conditional_order(
        &self,
        request: ConditionalOrderCreateRequest,
    ) -> Result<ConditionalOrderCreateResponse, SdkError> {
        self.post("/api/v1/conditional-orders", &request, true)
            .await
    }

    async fn get_conditional_orders(
        &self,
        status: OrderListStatus,
        symbol: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedConditionalOrderResponse, SdkError> {
        let mut query = vec![("status", status.as_str().to_string())];
        push_opt(&mut query, "symbol", symbol);
        push_opt(&mut query, "cursor", cursor);
        push_opt(&mut query, "limit", limit);
        self.get("/api/v1/conditional-orders", &query, true).await
    }

    async fn get_conditional_order(
        &self,
        conditional_order_id: &str,
    ) -> Result<ConditionalOrderDetailResponse, SdkError> {
        let path = format!("/api/v1/conditional-orders/{conditional_order_id}");
        self.get(&path, &[], true).await
    }

    async fn cancel_conditional_order(&self, conditional_order_id: &str) -> Result<(), SdkError> {
        let path = format!("/api/v1/conditional-orders/{conditional_order_id}");
        self.delete_void(&path, true).await
    }

    async fn modify_conditional_order(
        &self,
        conditional_order_id: &str,
        request: ConditionalOrderModifyRequest,
    ) -> Result<ConditionalOrderResponse, SdkError> {
        let path = format!("/api/v1/conditional-orders/{conditional_order_id}/modify");
        self.post(&path, &request, true).await
    }
}
