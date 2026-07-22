use crate::adapter::http_client::HttpClient;
use crate::adapter::push_opt;
use crate::domain::models::{
    BuyingPowerResponse, Commission, Currency, Order, OrderCreateRequest, OrderListStatus,
    OrderModifyRequest, OrderOperationResponse, OrderResponse, PaginatedOrderResponse,
    SellableQuantityResponse,
};
use crate::error::SdkError;
use crate::port::TradingPort;
use async_trait::async_trait;

#[async_trait]
impl TradingPort for HttpClient {
    async fn get_orders(
        &self,
        status: OrderListStatus,
        symbol: Option<&str>,
        from: Option<&str>,
        to: Option<&str>,
        cursor: Option<&str>,
        limit: Option<u32>,
    ) -> Result<PaginatedOrderResponse, SdkError> {
        let mut query = vec![("status", status.as_str().to_string())];
        push_opt(&mut query, "symbol", symbol);
        push_opt(&mut query, "from", from);
        push_opt(&mut query, "to", to);
        push_opt(&mut query, "cursor", cursor);
        push_opt(&mut query, "limit", limit);
        self.get("/api/v1/orders", &query, true).await
    }

    async fn create_order(&self, request: OrderCreateRequest) -> Result<OrderResponse, SdkError> {
        self.post("/api/v1/orders", &request, true).await
    }

    async fn get_order(&self, order_id: &str) -> Result<Order, SdkError> {
        let path = format!("/api/v1/orders/{order_id}");
        self.get(&path, &[], true).await
    }

    async fn modify_order(
        &self,
        order_id: &str,
        request: OrderModifyRequest,
    ) -> Result<OrderOperationResponse, SdkError> {
        let path = format!("/api/v1/orders/{order_id}/modify");
        self.post(&path, &request, true).await
    }

    async fn cancel_order(&self, order_id: &str) -> Result<OrderOperationResponse, SdkError> {
        let path = format!("/api/v1/orders/{order_id}/cancel");
        self.post_no_body(&path, true).await
    }

    async fn get_buying_power(&self, currency: Currency) -> Result<BuyingPowerResponse, SdkError> {
        let query = vec![("currency", currency.as_str().to_string())];
        self.get("/api/v1/buying-power", &query, true).await
    }

    async fn get_sellable_quantity(
        &self,
        symbol: &str,
    ) -> Result<SellableQuantityResponse, SdkError> {
        let query = vec![("symbol", symbol.to_string())];
        self.get("/api/v1/sellable-quantity", &query, true).await
    }

    async fn get_commissions(&self) -> Result<Vec<Commission>, SdkError> {
        self.get("/api/v1/commissions", &[], true).await
    }
}
