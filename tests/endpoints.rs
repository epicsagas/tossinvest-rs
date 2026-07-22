//! 엔드포인트 라우팅·인증·에러 회귀 테스트 (wiremock).
//!
//! portone-rs 의 테스트 패턴을 따르되, 토스증권 특유의 OAuth2 토큰 자동 발급과
//! `Authorization: Bearer` / `AccountSeq` 헤더 주입을 함께 검증합니다.

use serde_json::json;
use tossinvest_rs::v1::domain::models::{
    CandleInterval, ConditionRequest, ConditionalOrderCreateRequest, ConditionalOrderType,
    Currency, MarketCountry, OrderCreateRequest, OrderListStatus, OrderModifyRequest, OrderSide,
    OrderType, RankingDuration, RankingType, TimeInForce,
};
use tossinvest_rs::v1::{HttpClient, MarketDataPort, SdkError, TradingPort};
use wiremock::matchers::{body_string_contains, header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

const ACCOUNT_SEQ: i64 = 123456;

fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-id", "test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

fn account_client(base_url: String) -> HttpClient {
    client(base_url).with_account(ACCOUNT_SEQ)
}

/// `/oauth2/token` 발급을 mock. 모든 API 호출 전 자동으로 한 번 호출됩니다(캐싱).
async fn mount_token(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "test-token",
            "token_type": "Bearer",
            "expires_in": 86400
        })))
        .mount(server)
        .await;
}

// ---------- 인증 ----------

#[tokio::test]
async fn issue_token_calls_oauth2_endpoint() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .and(body_string_contains("grant_type=client_credentials"))
        .and(body_string_contains("client_id=test-id"))
        .and(body_string_contains("client_secret=test-secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "access_token": "abc",
            "token_type": "Bearer",
            "expires_in": 3600
        })))
        .mount(&server)
        .await;

    use tossinvest_rs::v1::AuthPort;
    let token = client(server.uri()).issue_token().await.unwrap();
    assert_eq!(token.access_token, "abc");
    assert_eq!(token.expires_in, 3600);
}

#[tokio::test]
async fn requests_carry_bearer_token() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/orderbook"))
        .and(header("authorization", "Bearer test-token"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {
                "timestamp": "2026-07-23T10:00:00+09:00",
                "currency": "KRW",
                "asks": [{"price": "70100", "volume": "10"}],
                "bids": [{"price": "69900", "volume": "20"}]
            }
        })))
        .mount(&server)
        .await;

    let orderbook = client(server.uri()).get_orderbook("005930").await.unwrap();
    assert_eq!(orderbook.currency, Currency::Krw);
    assert_eq!(orderbook.asks.len(), 1);
}

#[tokio::test]
async fn oauth2_error_maps_to_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .respond_with(ResponseTemplate::new(401).set_body_json(json!({
            "error": "invalid_client",
            "error_description": "Client authentication failed."
        })))
        .mount(&server)
        .await;

    let result = client(server.uri()).get_orderbook("005930").await;
    match result {
        Err(SdkError::ApiError { code, message, .. }) => {
            assert_eq!(code, "invalid_client");
            assert_eq!(message, "Client authentication failed.");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

// ---------- 마켓 데이터 ----------

#[tokio::test]
async fn get_prices_unwraps_result_array() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/prices"))
        .and(query_param("symbols", "005930,005935"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({
                "result": [
                    {"symbol":"005930","timestamp":"2026-07-23T10:00:00+09:00","lastPrice":"70000","currency":"KRW"},
                    {"symbol":"005935","timestamp":"2026-07-23T10:00:00+09:00","lastPrice":"160000","currency":"KRW"}
                ]
            })),
        )
        .mount(&server)
        .await;

    let prices = client(server.uri())
        .get_prices(&["005930", "005935"])
        .await
        .unwrap();
    assert_eq!(prices.len(), 2);
    assert_eq!(prices[0].symbol, "005930");
    assert_eq!(prices[0].lastPrice, "70000");
}

#[tokio::test]
async fn get_candles_builds_query_params() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/candles"))
        .and(query_param("symbol", "005930"))
        .and(query_param("interval", "1d"))
        .and(query_param("count", "5"))
        .and(query_param("adjusted", "true"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {
                "candles": [{
                    "timestamp":"2026-07-22T00:00:00+09:00",
                    "openPrice":"69000","highPrice":"71000","lowPrice":"68500","closePrice":"70000",
                    "volume":"1000000","currency":"KRW"
                }],
                "nextBefore": "2026-07-21T00:00:00+09:00"
            }
        })))
        .mount(&server)
        .await;

    let candles = client(server.uri())
        .get_candles("005930", CandleInterval::OneDay, Some(5), None, Some(true))
        .await
        .unwrap();
    assert_eq!(candles.candles.len(), 1);
    assert_eq!(
        candles.nextBefore.as_deref(),
        Some("2026-07-21T00:00:00+09:00")
    );
}

#[tokio::test]
async fn get_stock_warnings_uses_path_param() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/stocks/005930/warnings"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({
                "result": [{"warningType":"MANAGEMENT","exchange":"KRX","startDate":"2026-01-01","endDate":null}]
            })),
        )
        .mount(&server)
        .await;

    let warnings = client(server.uri())
        .get_stock_warnings("005930")
        .await
        .unwrap();
    assert_eq!(warnings.len(), 1);
    assert_eq!(warnings[0].warningType, "MANAGEMENT");
}

#[tokio::test]
async fn get_rankings_builds_enum_query() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/rankings"))
        .and(query_param("type", "TOP_GAINERS"))
        .and(query_param("marketCountry", "KR"))
        .and(query_param("duration", "1d"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {"rankedAt":"2026-07-23T15:30:00+09:00","rankings":[]}
        })))
        .mount(&server)
        .await;

    let rankings = client(server.uri())
        .get_rankings(
            RankingType::TopGainers,
            MarketCountry::Kr,
            RankingDuration::OneDay,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(rankings.rankings.len(), 0);
}

#[tokio::test]
async fn market_data_404_maps_to_api_error() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/orderbook"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(json!({
                "error": {"requestId":"req-1","code":"symbol-not-found","message":"종목을 찾을 수 없습니다."}
            })),
        )
        .mount(&server)
        .await;

    let result = client(server.uri()).get_orderbook("000000").await;
    match result {
        Err(SdkError::ApiError {
            status,
            code,
            message,
            request_id,
        }) => {
            assert_eq!(status, reqwest::StatusCode::NOT_FOUND);
            assert_eq!(code, "symbol-not-found");
            assert_eq!(message, "종목을 찾을 수 없습니다.");
            assert_eq!(request_id.as_deref(), Some("req-1"));
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}

// ---------- 계좌·잔고 ----------

#[tokio::test]
async fn get_accounts_requires_no_account_header() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/accounts"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": [{"accountNo":"1234-5678","accountSeq":123456,"accountType":"BROKERAGE"}]
        })))
        .mount(&server)
        .await;

    use tossinvest_rs::v1::AccountPort;
    let accounts = client(server.uri()).get_accounts().await.unwrap();
    assert_eq!(accounts.len(), 1);
    assert_eq!(accounts[0].accountSeq, 123456);
}

#[tokio::test]
async fn get_holdings_sends_account_seq_header() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/holdings"))
        .and(header("AccountSeq", "123456"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({
                "result": {
                    "totalPurchaseAmount": {"krw":"1000000","usd":null},
                    "marketValue": {"amount":{"krw":"1200000","usd":null},"amountAfterCost":{"krw":"1180000","usd":null}},
                    "profitLoss": {"amount":{"krw":"200000","usd":null},"amountAfterCost":{"krw":"180000","usd":null},"rate":"0.2","rateAfterCost":"0.18"},
                    "dailyProfitLoss": {"amount":{"krw":"10000","usd":null},"rate":"0.01"},
                    "items": []
                }
            })),
        )
        .mount(&server)
        .await;

    use tossinvest_rs::v1::AccountPort;
    let holdings = account_client(server.uri())
        .get_holdings(None)
        .await
        .unwrap();
    assert!(holdings.items.is_empty());
}

#[tokio::test]
async fn account_endpoint_without_account_seq_errors() {
    let server = MockServer::start().await;
    mount_token(&server).await;

    use tossinvest_rs::v1::AccountPort;
    let result = client(server.uri()).get_holdings(None).await;
    assert!(
        matches!(result, Err(SdkError::Unknown(_))),
        "got {result:?}"
    );
}

// ---------- 주문 ----------

#[tokio::test]
async fn create_order_serializes_amount_based_body() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("POST"))
        .and(path("/api/v1/orders"))
        .and(header("AccountSeq", "123456"))
        .and(body_string_contains("\"symbol\":\"005930\""))
        .and(body_string_contains("\"side\":\"BUY\""))
        .and(body_string_contains("\"orderType\":\"MARKET\""))
        .and(body_string_contains("\"orderAmount\":\"1000000\""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {"orderId":"ord-1","clientOrderId":null}
        })))
        .mount(&server)
        .await;

    let req = OrderCreateRequest::amount_based("005930", OrderSide::Buy, "1000000");
    let resp = account_client(server.uri())
        .create_order(req)
        .await
        .unwrap();
    assert_eq!(resp.orderId, "ord-1");
}

#[tokio::test]
async fn create_order_serializes_quantity_based_with_optional_fields() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("POST"))
        .and(path("/api/v1/orders"))
        .and(body_string_contains("\"quantity\":\"10\""))
        .and(body_string_contains("\"price\":\"70000\""))
        .and(body_string_contains("\"timeInForce\":\"DAY\""))
        .and(body_string_contains("\"clientOrderId\":\"my-id\""))
        // orderAmount 는 수량 기반 주문에 포함되지 않아야 합니다.
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"result":{"orderId":"ord-2"}})),
        )
        .mount(&server)
        .await;

    let req = OrderCreateRequest::quantity_based("005930", OrderSide::Buy, OrderType::Limit, "10")
        .price("70000")
        .time_in_force(TimeInForce::Day)
        .client_order_id("my-id");
    let resp = account_client(server.uri())
        .create_order(req)
        .await
        .unwrap();
    assert_eq!(resp.orderId, "ord-2");
}

#[tokio::test]
async fn cancel_order_returns_response_without_body() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("POST"))
        .and(path("/api/v1/orders/ord-1/cancel"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"result":{"orderId":"ord-1"}})),
        )
        .mount(&server)
        .await;

    let resp = account_client(server.uri())
        .cancel_order("ord-1")
        .await
        .unwrap();
    assert_eq!(resp.orderId, "ord-1");
}

#[tokio::test]
async fn get_orders_builds_query() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/orders"))
        .and(query_param("status", "OPEN"))
        .and(query_param("symbol", "005930"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {"orders":[],"nextCursor":null,"hasNext":false}
        })))
        .mount(&server)
        .await;

    let page = account_client(server.uri())
        .get_orders(
            OrderListStatus::Open,
            Some("005930"),
            None,
            None,
            None,
            Some(10),
        )
        .await
        .unwrap();
    assert!(!page.hasNext);
}

#[tokio::test]
async fn modify_order_serializes_body() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("POST"))
        .and(path("/api/v1/orders/ord-1/modify"))
        .and(body_string_contains("\"orderType\":\"LIMIT\""))
        .and(body_string_contains("\"quantity\":\"5\""))
        .and(body_string_contains("\"price\":\"71000\""))
        .and(body_string_contains("\"confirmHighValueOrder\":true"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(json!({"result":{"orderId":"ord-1"}})),
        )
        .mount(&server)
        .await;

    let req = OrderModifyRequest::new(OrderType::Limit, "5", "71000", true);
    let resp = account_client(server.uri())
        .modify_order("ord-1", req)
        .await
        .unwrap();
    assert_eq!(resp.orderId, "ord-1");
}

// ---------- 조건부 주문 ----------

#[tokio::test]
async fn create_conditional_order_serializes_first_second() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("POST"))
        .and(path("/api/v1/conditional-orders"))
        .and(body_string_contains("\"type\":\"OCO\""))
        .and(body_string_contains("\"first\":"))
        .and(body_string_contains("\"second\":"))
        .and(body_string_contains("\"orderSide\":\"BUY\""))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "result": {"conditionalOrderId":"co-1","clientOrderId":null}
        })))
        .mount(&server)
        .await;

    let req = ConditionalOrderCreateRequest::new(
        "005930",
        ConditionalOrderType::Oco,
        "10",
        OrderType::Limit,
        "2026-12-31",
        ConditionRequest::new(OrderSide::Buy, "70000", "70100"),
    )
    .second(ConditionRequest::new(OrderSide::Buy, "65000", "65100"));

    use tossinvest_rs::v1::ConditionalOrderPort;
    let resp = account_client(server.uri())
        .create_conditional_order(req)
        .await
        .unwrap();
    assert_eq!(resp.conditionalOrderId, "co-1");
}

#[tokio::test]
async fn cancel_conditional_order_accepts_204() {
    let server = MockServer::start().await;
    mount_token(&server).await;
    Mock::given(method("DELETE"))
        .and(path("/api/v1/conditional-orders/co-1"))
        .and(header("AccountSeq", "123456"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    use tossinvest_rs::v1::ConditionalOrderPort;
    account_client(server.uri())
        .cancel_conditional_order("co-1")
        .await
        .expect("204 should map to Ok(())");
}
