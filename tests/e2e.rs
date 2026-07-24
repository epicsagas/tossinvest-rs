//! 전체 통합 E2E 시나리오 테스트 (wiremock + 목업 데이터).
//!
//! 개별 엔드포인트 회귀(`endpoints.rs`)와 달리, 실제 사용 워크플로우를 단일 클라이언트·
//! 단일 목업 서버에서 순차 호출해 인증 흐름·데이터 전달·페이지네이션·에러 전파를 종합 검증합니다.

mod common;
use common::{account_client, client, fixtures, mock_token, mock_token_once, ACCOUNT_SEQ};

use tossinvest_sdk::v1::domain::models::{
    CandleInterval, ConditionRequest, ConditionalOrderCreateRequest, ConditionalOrderModifyRequest,
    ConditionalOrderType, MarketCountry, OrderCreateRequest, OrderListStatus, OrderSide, OrderType,
    RankingDuration, RankingType,
};
use tossinvest_sdk::v1::{
    AccountPort, ConditionalOrderPort, MarketDataPort, SdkError, TradingPort,
};
use wiremock::matchers::{method, path, query_param, query_param_is_missing};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// `GET` 200 응답 mock (path 만 매칭).
async fn mock_get(server: &MockServer, p: &str, body: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path(p))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

/// `POST` 200 응답 mock (path 만 매칭, 본문 미검증).
async fn mock_post(server: &MockServer, p: &str, body: serde_json::Value) {
    Mock::given(method("POST"))
        .and(path(p))
        .respond_with(ResponseTemplate::new(200).set_body_json(body))
        .mount(server)
        .await;
}

// ---------- 인증 ----------

#[tokio::test]
async fn auth_token_reused_across_calls() {
    let server = MockServer::start().await;
    // 토큰을 정확히 1회만 발급하도록 검증. 캐싱 없이 재발급되면 서버 drop 시 panic.
    mock_token_once(&server).await;
    mock_get(&server, "/api/v1/prices", fixtures::price_list(&["005930"])).await;
    mock_get(&server, "/api/v1/orderbook", fixtures::orderbook()).await;
    mock_get(&server, "/api/v1/trades", fixtures::trades(3)).await;
    mock_get(&server, "/api/v1/candles", fixtures::candles(3)).await;
    mock_get(&server, "/api/v1/stocks", fixtures::stocks(&["005930"])).await;

    let client = client(server.uri());
    client.get_prices(&["005930"]).await.unwrap();
    client.get_orderbook("005930").await.unwrap();
    client.get_trades("005930", None).await.unwrap();
    client
        .get_candles("005930", CandleInterval::OneDay, Some(3), None, None)
        .await
        .unwrap();
    client.get_stocks(&["005930"]).await.unwrap();
    // (서버 drop 시 expect(1) 위반 여부 검증)
}

// ---------- 투자자 매수 여정 (KR) ----------

#[tokio::test]
async fn investor_buy_journey_kr() {
    let server = MockServer::start().await;
    mock_token(&server).await;
    mock_get(&server, "/api/v1/prices", fixtures::price_list(&["005930"])).await;
    mock_get(&server, "/api/v1/orderbook", fixtures::orderbook()).await;
    mock_get(&server, "/api/v1/accounts", fixtures::accounts()).await;
    mock_get(&server, "/api/v1/holdings", fixtures::holdings_overview()).await;
    mock_get(&server, "/api/v1/orders/ord-1", fixtures::order("ord-1")).await;
    mock_post(&server, "/api/v1/orders", fixtures::created_order("ord-1")).await;
    mock_post(
        &server,
        "/api/v1/orders/ord-1/cancel",
        fixtures::order_operation("ord-1"),
    )
    .await;

    let trader = account_client(server.uri());

    // 시세 → 호가
    let prices = trader.get_prices(&["005930"]).await.unwrap();
    assert_eq!(prices[0].symbol, "005930");
    let orderbook = trader.get_orderbook("005930").await.unwrap();
    assert!(!orderbook.asks.is_empty());

    // 계좌 확인 → accountSeq 일관성 → 잔고
    let accounts = trader.get_accounts().await.unwrap();
    assert_eq!(accounts[0].accountSeq, ACCOUNT_SEQ);
    let holdings = trader.get_holdings(None).await.unwrap();
    assert_eq!(holdings.items.len(), 1);

    // 매수 주문 → 주문 확인 → 취소 (orderId 전달)
    let request =
        OrderCreateRequest::quantity_based("005930", OrderSide::Buy, OrderType::Limit, "10")
            .price("70000");
    let created = trader.create_order(request).await.unwrap();
    assert_eq!(created.orderId, "ord-1");

    let detail = trader.get_order("ord-1").await.unwrap();
    assert_eq!(detail.orderId, "ord-1");
    assert_eq!(detail.execution.filledQuantity, "10");

    let canceled = trader.cancel_order("ord-1").await.unwrap();
    assert_eq!(canceled.orderId, "ord-1");
}

// ---------- 주문 목록 페이지네이션 순회 ----------

#[tokio::test]
async fn orders_pagination_walk() {
    let server = MockServer::start().await;
    mock_token(&server).await;

    // 1페이지: cursor 없음 → 2건 + nextCursor
    Mock::given(method("GET"))
        .and(path("/api/v1/orders"))
        .and(query_param("status", "OPEN"))
        .and(query_param_is_missing("cursor"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(fixtures::orders_page(
                &[fixtures::order_summary("a"), fixtures::order_summary("b")],
                Some("cursor-2"),
                true,
            )),
        )
        .mount(&server)
        .await;
    // 2페이지: cursor=cursor-2 → 1건, 끝
    Mock::given(method("GET"))
        .and(path("/api/v1/orders"))
        .and(query_param("cursor", "cursor-2"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(fixtures::orders_page(
                &[fixtures::order_summary("c")],
                None,
                false,
            )),
        )
        .mount(&server)
        .await;

    let trader = account_client(server.uri());

    let page1 = trader
        .get_orders(OrderListStatus::Open, None, None, None, None, None)
        .await
        .unwrap();
    assert!(page1.hasNext);
    assert_eq!(page1.nextCursor.as_deref(), Some("cursor-2"));
    assert_eq!(page1.orders.len(), 2);

    let cursor = page1.nextCursor.unwrap();
    let page2 = trader
        .get_orders(OrderListStatus::Open, None, None, None, Some(&cursor), None)
        .await
        .unwrap();
    assert!(!page2.hasNext);
    assert!(page2.nextCursor.is_none());
    assert_eq!(page2.orders.len(), 1);
    assert_eq!(page2.orders[0].orderId, "c");
}

// ---------- 조건주문(OCO) 생명주기 ----------

#[tokio::test]
async fn conditional_order_oco_lifecycle() {
    let server = MockServer::start().await;
    mock_token(&server).await;
    mock_post(
        &server,
        "/api/v1/conditional-orders",
        fixtures::created_conditional_order("co-1"),
    )
    .await;
    mock_get(
        &server,
        "/api/v1/conditional-orders/co-1",
        fixtures::conditional_order_detail("co-1", "OCO"),
    )
    .await;
    mock_post(
        &server,
        "/api/v1/conditional-orders/co-1/modify",
        fixtures::conditional_order_response("co-1"),
    )
    .await;
    Mock::given(method("DELETE"))
        .and(path("/api/v1/conditional-orders/co-1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;

    let trader = account_client(server.uri());

    let first = ConditionRequest::new(OrderSide::Buy, "65000", "65100");
    let second = ConditionRequest::new(OrderSide::Buy, "77000", "77100");
    let create = ConditionalOrderCreateRequest::new(
        "005930",
        ConditionalOrderType::Oco,
        "10",
        OrderType::Limit,
        "2026-12-31",
        first,
    )
    .second(second);

    let created = trader.create_conditional_order(create).await.unwrap();
    assert_eq!(created.conditionalOrderId, "co-1");

    let detail = trader.get_conditional_order("co-1").await.unwrap();
    assert_eq!(detail.conditional_type, ConditionalOrderType::Oco);
    assert!(detail.second.is_some()); // OCO 는 second 존재

    let modify = ConditionalOrderModifyRequest::new(
        ConditionalOrderType::Oco,
        "10",
        OrderType::Limit,
        "2026-12-31",
        ConditionRequest::new(OrderSide::Buy, "64000", "64100"),
    )
    .second(ConditionRequest::new(OrderSide::Buy, "78000", "78100"));
    let modified = trader
        .modify_conditional_order("co-1", modify)
        .await
        .unwrap();
    assert_eq!(modified.conditionalOrderId, "co-1");

    trader
        .cancel_conditional_order("co-1")
        .await
        .expect("204 should map to Ok(())");
}

// ---------- 시장 데이터 배치 세션 ----------

#[tokio::test]
async fn market_data_batch_session() {
    let server = MockServer::start().await;
    mock_token(&server).await;
    mock_get(
        &server,
        "/api/v1/stocks",
        fixtures::stocks(&["005930", "005935"]),
    )
    .await;
    mock_get(&server, "/api/v1/candles", fixtures::candles(5)).await;
    mock_get(&server, "/api/v1/rankings", fixtures::rankings()).await;

    let client = client(server.uri());

    let stocks = client.get_stocks(&["005930", "005935"]).await.unwrap();
    assert_eq!(stocks.len(), 2);
    assert_eq!(
        stocks[0].currency,
        tossinvest_sdk::v1::domain::models::Currency::Krw
    );

    let candles = client
        .get_candles("005930", CandleInterval::OneDay, Some(5), None, None)
        .await
        .unwrap();
    assert_eq!(candles.candles.len(), 5);

    let rankings = client
        .get_rankings(
            RankingType::TopGainers,
            MarketCountry::Kr,
            RankingDuration::OneDay,
            None,
            None,
        )
        .await
        .unwrap();
    assert_eq!(rankings.rankings.len(), 1);
    assert_eq!(rankings.rankings[0].rank, 1);
}

// ---------- 비즈니스 에러가 흐름을 중단 ----------

#[tokio::test]
async fn business_error_aborts_flow() {
    let server = MockServer::start().await;
    mock_token(&server).await;
    mock_get(&server, "/api/v1/prices", fixtures::price_list(&["005930"])).await;
    Mock::given(method("GET"))
        .and(path("/api/v1/orderbook"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(fixtures::business_error(
                "symbol-not-found",
                "종목을 찾을 수 없습니다.",
            )),
        )
        .mount(&server)
        .await;

    let client = client(server.uri());

    // 정상 호출
    let prices = client.get_prices(&["005930"]).await.unwrap();
    assert_eq!(prices.len(), 1);

    // 존재하지 않는 종목 → 비즈니스 에러 전파
    let result = client.get_orderbook("000000").await;
    match result {
        Err(SdkError::ApiError { code, message, .. }) => {
            assert_eq!(code, "symbol-not-found");
            assert_eq!(message, "종목을 찾을 수 없습니다.");
        }
        other => panic!("expected ApiError, got {other:?}"),
    }
}
