//! 통합 테스트 공유 헬퍼와 목업 데이터 fixtures.
//!
//! 각 통합 테스트 바이너리는 `mod common; use common::*;` 로 가져옵니다.
//! fixtures 는 토스증권 Open API 응답 형식(`{"result": T}` envelope · decimal string ·
//! nullable · pagination meta)을 코드 내에서 모방합니다 (외부 JSON 파일 I/O 없음).

// 테스트 인프라: 일부 fixtures 는 특정 테스트 바이너리에서만 사용되어 dead_code 경고를 허용합니다.
#![allow(dead_code)]

use serde_json::{json, Value};
use tossinvest_sdk::v1::HttpClient;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// `with_account` 헤더로 사용할 기본 accountSeq.
pub const ACCOUNT_SEQ: i64 = 123456;

pub fn client(base_url: String) -> HttpClient {
    HttpClient::new("test-id", "test-secret")
        .expect("Failed to initialize HttpClient")
        .with_base_url(base_url)
}

pub fn account_client(base_url: String) -> HttpClient {
    client(base_url).with_account(ACCOUNT_SEQ)
}

fn token_response(expires_in: i64) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(json!({
        "access_token": "test-token",
        "token_type": "Bearer",
        "expires_in": expires_in
    }))
}

/// `/oauth2/token` 발급을 mock (횟수 제한 없음). 캐싱 검증이 불필요한 일반 테스트용.
pub async fn mock_token(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .respond_with(token_response(86400))
        .mount(server)
        .await;
}

/// `/oauth2/token` 이 **정확히 1회**만 호출됨을 검증 — 토큰 캐싱 재사용 증명.
/// 여러 API 호출이 같은 토큰을 재사용하지 않으면 wiremock 이 테스트를 실패시킵니다.
pub async fn mock_token_once(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/oauth2/token"))
        .respond_with(token_response(86400))
        .expect(1)
        .mount(server)
        .await;
}

// ---- fixtures ----

pub mod fixtures {
    use super::{json, Value};

    fn wrap(result: Value) -> Value {
        json!({ "result": result })
    }

    /// 여러 종목의 현재가 목록.
    pub fn price_list(symbols: &[&str]) -> Value {
        let items: Vec<Value> = symbols
            .iter()
            .enumerate()
            .map(|(i, s)| {
                json!({
                    "symbol": s,
                    "timestamp": "2026-07-23T10:00:00+09:00",
                    "lastPrice": format!("{}", 70000 + i as i64 * 1000),
                    "currency": "KRW"
                })
            })
            .collect();
        wrap(Value::Array(items))
    }

    /// 단일 종목 호가창.
    pub fn orderbook() -> Value {
        wrap(json!({
            "timestamp": "2026-07-23T10:00:00+09:00",
            "currency": "KRW",
            "asks": [{"price":"70100","volume":"10"},{"price":"70200","volume":"5"}],
            "bids": [{"price":"69900","volume":"20"},{"price":"69800","volume":"8"}]
        }))
    }

    /// `n`개 캔들 시계열.
    pub fn candles(n: usize) -> Value {
        let candles: Vec<Value> = (0..n)
            .map(|i| {
                json!({
                    "timestamp": format!("2026-07-{:02}T00:00:00+09:00", 23 - i),
                    "openPrice":"69000","highPrice":"71000","lowPrice":"68500","closePrice": format!("{}", 70000 - i),
                    "volume":"1000000","currency":"KRW"
                })
            })
            .collect();
        wrap(json!({ "candles": candles, "nextBefore": "2026-07-10T00:00:00+09:00" }))
    }

    /// `n`개 체결 내역 (Trade: price/volume/timestamp/currency).
    pub fn trades(n: usize) -> Value {
        let items: Vec<Value> = (0..n)
            .map(|i| {
                json!({
                    "price": format!("{}", 70000 + i),
                    "volume": format!("{}", 100 - i),
                    "timestamp": "2026-07-23T10:00:00+09:00",
                    "currency": "KRW"
                })
            })
            .collect();
        wrap(Value::Array(items))
    }

    /// 여러 종목 정보.
    pub fn stocks(symbols: &[&str]) -> Value {
        let items: Vec<Value> = symbols
            .iter()
            .map(|s| {
                json!({
                    "symbol": s, "name": format!("{s} 이름"), "englishName": format!("{s} EN"),
                    "isinCode":"KR7005930003", "market":"KRX", "securityType":"STOCK",
                    "isCommonShare": true, "status":"ACTIVE", "currency":"KRW",
                    "sharesOutstanding":"1000000"
                })
            })
            .collect();
        wrap(Value::Array(items))
    }

    /// 순위 응답.
    pub fn rankings() -> Value {
        wrap(json!({
            "rankedAt": "2026-07-23T15:30:00+09:00",
            "rankings": [
                {"rank":1,"symbol":"005930","currency":"KRW",
                 "price":{"lastPrice":"70000","basePrice":"69000","changeRate":"0.0145"},
                 "tradingVolume":"18432100","tradingAmount":"1286247000000"}
            ]
        }))
    }

    /// 보유 계좌 목록 (accountSeq 는 [`super::ACCOUNT_SEQ`]).
    pub fn accounts() -> Value {
        wrap(json!([
            {"accountNo":"1234-5678","accountSeq": super::ACCOUNT_SEQ, "accountType":"BROKERAGE"}
        ]))
    }

    /// 보유 종목 종합.
    pub fn holdings_overview() -> Value {
        wrap(json!({
            "totalPurchaseAmount": {"krw":"1000000","usd":null},
            "marketValue": {"amount":{"krw":"1200000","usd":null},"amountAfterCost":{"krw":"1180000","usd":null}},
            "profitLoss": {"amount":{"krw":"200000","usd":null},"amountAfterCost":{"krw":"180000","usd":null},"rate":"0.2","rateAfterCost":"0.18"},
            "dailyProfitLoss": {"amount":{"krw":"10000","usd":null},"rate":"0.01"},
            "items": [{
                "symbol":"005930","name":"삼성전자","marketCountry":"KR","currency":"KRW",
                "quantity":"10","lastPrice":"70000","averagePurchasePrice":"65000",
                "marketValue":{"purchaseAmount":"650000","amount":"700000","amountAfterCost":"690000"},
                "profitLoss":{"amount":"50000","amountAfterCost":"40000","rate":"0.077","rateAfterCost":"0.062"},
                "dailyProfitLoss":{"amount":"5000","rate":"0.007"},
                "cost":{"commission":"1000","tax":null}
            }]
        }))
    }

    /// 주문 한 건 (체결 포함).
    pub fn order(order_id: &str) -> Value {
        wrap(json!({
            "orderId": order_id, "symbol":"005930", "side":"BUY", "orderType":"LIMIT",
            "timeInForce":"DAY", "status":"FILLED", "price":"70000", "quantity":"10",
            "orderAmount":null, "currency":"KRW", "orderedAt":"2026-07-23T10:00:00+09:00",
            "canceledAt":null,
            "execution": {
                "filledQuantity":"10","averageFilledPrice":"70000","filledAmount":"700000",
                "commission":"70","tax":"0","filledAt":"2026-07-23T10:00:01+09:00","settlementDate":"2026-07-25"
            }
        }))
    }

    /// 주문 생성 응답.
    pub fn created_order(order_id: &str) -> Value {
        wrap(json!({ "orderId": order_id, "clientOrderId": "my-order-001" }))
    }

    /// 주문 정정·취소 응답.
    pub fn order_operation(order_id: &str) -> Value {
        wrap(json!({ "orderId": order_id }))
    }

    /// 주문 목록 페이지.
    pub fn orders_page(orders: &[Value], next_cursor: Option<&str>, has_next: bool) -> Value {
        wrap(json!({
            "orders": orders,
            "nextCursor": next_cursor,
            "hasNext": has_next
        }))
    }

    /// 단일 주문 항목 (목록용).
    pub fn order_summary(order_id: &str) -> Value {
        json!({
            "orderId": order_id, "symbol":"005930", "side":"BUY", "orderType":"LIMIT",
            "timeInForce":"DAY", "status":"FILLED", "price":"70000", "quantity":"10",
            "orderAmount":null, "currency":"KRW", "orderedAt":"2026-07-23T10:00:00+09:00",
            "canceledAt":null,
            "execution": {"filledQuantity":"10","averageFilledPrice":"70000","filledAmount":"700000",
                          "commission":"70","tax":"0","filledAt":"2026-07-23T10:00:01+09:00","settlementDate":"2026-07-25"}
        })
    }

    /// 조건주문 상세 (OCO). `first`/`second` 감시 조건 포함.
    pub fn conditional_order_detail(id: &str, cond_type: &str) -> Value {
        wrap(json!({
            "conditionalOrderId": id, "type": cond_type, "status":"WATCHING",
            "symbol":"005930", "market":"KR", "quantity":"10", "orderType":"LIMIT",
            "expireDate":"2026-12-31",
            "first": {"type":"STOP","status":"WATCHING","triggerPrice":"65000","targetProfitRate":null,"orderPrice":"65100","triggeredOrderId":null},
            "second": {"type":"PROFIT_RATE","status":"WATCHING","triggerPrice":null,"targetProfitRate":"0.1","orderPrice":"77000","triggeredOrderId":null},
            "createdAt":"2026-07-23T09:00:00+09:00"
        }))
    }

    /// 조건주문 생성 응답.
    pub fn created_conditional_order(id: &str) -> Value {
        wrap(json!({ "conditionalOrderId": id, "clientOrderId": null }))
    }

    /// 조건주문 수정 응답.
    pub fn conditional_order_response(id: &str) -> Value {
        wrap(json!({ "conditionalOrderId": id }))
    }

    /// 비즈니스 에러 envelope.
    pub fn business_error(code: &str, message: &str) -> Value {
        json!({ "error": {"requestId":"req-1","code":code,"message":message} })
    }
}
