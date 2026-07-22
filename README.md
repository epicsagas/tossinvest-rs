# Toss Securities (토스증권) Rust SDK (Unofficial)

> ⚠️ **중요 공지 (Disclaimer)**
> 본 라이브러리는 개인이 개발한 **비공식(Unofficial)** Rust SDK입니다.
> [토스증권](https://tossinvest.com)의 공식 지원이나 관리를 받지 않는 독립적인 오픈소스 프로젝트입니다.
> 공식 정보 및 문서는 [토스증권 Open API 문서](https://developers.tossinvest.com/)를 참고하세요.

---

> 원본 API 명세: 토스증권 Open API v1.2.4 (OpenAPI 3.1.0)

토스증권 Open API를 위한 비공식 Rust SDK입니다. **헥사고날 아키텍처(포트와 어댑터 패턴)** 로 설계되어 도메인 로직과 HTTP 전송 계층이 깔끔하게 분리되어 있습니다. 시세·호가·체결·캔들·종목정보·순위·환율·시장캘린더·투자자매매동향 등 시장 데이터와 계좌·잔고·주문·조건부주문까지 29개 엔드포인트를 일관된 API로 제공합니다.

## 주요 기능

- **OAuth2 자동 인증**: Client Credentials 토큰을 발급받아 캐싱하고, 만료 시 자동 갱신하여 `Authorization: Bearer` 헤더로 주입합니다.
- **계좌 헤더 자동 처리**: 계좌 기반 엔드포인트(잔고·주문·조건부주문 등)에 필요한 `AccountSeq` 헤더를 `with_account()` 로 자동 구성합니다.
- **단일 에러 타입**: 모든 호출이 `SdkError` 하나만 반환 — 초기화·인증·API 에러를 통합.
- **도메인별 포트**: 시장데이터·계좌·주문·조건부주문을 각각의 trait으로 분리.
- **전수 회귀 테스트**: wiremock 기반으로 라우팅·인증·에러 매핑을 검증.

## 설치

`Cargo.toml`에 추가하세요:

```toml
[dependencies]
tossinvest-rs = "0.1.0"
```

## 빠른 시작

발급받은 클라이언트 자격증명(`client_id`, `client_secret`)으로 `HttpClient`를 초기화하면 모든 포트 trait 메서드를 사용할 수 있습니다.

```rust
use tossinvest_rs::v1::{HttpClient, MarketDataPort};

#[tokio::main]
async fn main() -> Result<(), tossinvest_rs::v1::SdkError> {
    // OAuth2 액세스 토큰이 자동으로 발급되어 Authorization 헤더에 주입됩니다.
    let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?;

    // 시장 데이터 조회 (계좌 불필요).
    let prices = client.get_prices(&["005930"]).await?;
    println!("현재가: {prices:?}");

    Ok(())
}
```

> 위 예제는 [`examples/quickstart.rs`](examples/quickstart.rs)와 동일하며, CI에서 `cargo build --examples`로 항상 컴파일됨을 보장합니다.

## 인증 (OAuth2 Client Credentials)

토스증권 Open API는 OAuth2 Client Credentials 그랜트를 사용합니다. `HttpClient::new(client_id, client_secret)`는 토큰을 자동 발급·캐싱하며, 만료(여유분 60초 전) 시점에 자동 갱신합니다.

```rust
let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    .with_base_url("https://openapi.tossinvest.com"); // 기본값, 필요 시 오버라이드
```

`with_base_url`은 테스트(mock 서버)나 사설 엔드포인트에 연결할 때 사용합니다.

토큰을 명시적으로 발급받거나 만료 정보를 확인하려면 `AuthPort`를 사용합니다 (일반적으로는 불필요).

```rust
use tossinvest_rs::v1::AuthPort;
let token = client.issue_token().await?;
println!("만료까지 {}초", token.expires_in);
```

## 계좌 선택 (AccountSeq)

잔고·주문·조건부주문·매수가능금액·매도가능수량·수수료 엔드포인트는 `AccountSeq` 헤더가 필요합니다. `with_account(accountSeq)`로 기본 계좌를 지정하세요. `accountSeq`는 `GET /accounts` 응답에서 얻습니다.

```rust
use tossinvest_rs::v1::{HttpClient, AccountPort, TradingPort};

let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?;

// 1. 보유 계좌 목록에서 accountSeq 확인
let accounts = client.get_accounts().await?;
let seq = accounts[0].accountSeq;

// 2. 기본 계좌로 설정
let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    .with_account(seq);

// 3. 계좌 기반 API 호출
let holdings = client.get_holdings(None).await?;
```

## 도메인 포트

`HttpClient`는 5개의 포트 trait을 구현합니다. 각 trait은 `tossinvest_rs::v1::` 최상위에서 직접 임포트할 수 있습니다.

| 포트 Trait | 주요 기능 |
| --- | --- |
| `MarketDataPort` | 현재가·호가·체결·캔들·가격제한·종목정보·투자유의·환율·시장캘린더·순위·시장지표·투자자매매동향 |
| `AccountPort` | 보유 계좌 목록, 보유 종목 종합 |
| `TradingPort` | 주문(생성/조회/정정/취소), 매수가능금액, 매도가능수량, 수수료율 |
| `ConditionalOrderPort` | 조건부(자동)주문 생성/조회/수정/취소 (SINGLE/OCO/OTO) |
| `AuthPort` | OAuth2 토큰 수동 발급 |

```rust
use tossinvest_rs::v1::{
    HttpClient, MarketDataPort, TradingPort,
    domain::models::{Currency, OrderCreateRequest, OrderSide, OrderType},
};

let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
    .with_account(123456);

// 시장가 매수 (금액 기반)
let order = client.create_order(
    OrderCreateRequest::amount_based("005930", OrderSide::Buy, "1000000")
).await?;

// 매수 가능 금액
let power = client.get_buying_power(Currency::Krw).await?;
```

## 에러 처리

모든 오류는 단일 `SdkError`로 표현됩니다.

```rust
use tossinvest_rs::v1::SdkError;

match client.get_prices(&["000000"]).await {
    Ok(prices) => println!("{prices:?}"),
    Err(SdkError::ApiError { status, code, message, request_id }) => {
        // 토스증권 API가 반환한 에러 (status, code, message, requestId)
        println!("API 에러 {status}: {code} - {message} (request={request_id:?})");
    }
    Err(SdkError::HttpError(e)) => eprintln!("전송 계층 오류: {e}"),
    Err(e) => eprintln!("기타 오류: {e}"),
}
```

`ApiError.code`로 특정 에러 케이스를 매칭할 수 있습니다 (예: `order-not-found`, `invalid_client`).

| Variant | 의미 |
|---------|------|
| `HttpError` | reqwest 전송 계층 오류(타임아웃, 연결 실패 등) |
| `InvalidHeaderValue` | 헤더에 사용할 수 없는 문자 포함 |
| `SerializationError` | 요청/응답 직렬화 실패 |
| `ApiError { status, code, message, request_id }` | 토스증권 API가 반환한 비-2xx 응답 |
| `Unknown` | 분류되지 않은 오류 (예: `AccountSeq` 미설정) |

## 아키텍처 (Hexagonal / Ports & Adapters)

```
src/
├── domain/models/      # OpenAPI 명세의 72개 데이터 구조 (serde)
├── port/               # 도메인별 trait (추상 경계)
│   ├── market_data_port.rs
│   ├── trading_port.rs
│   └── ...
├── adapter/
│   ├── http_client.rs  # 포트 trait의 구체적 구현 (입구)
│   ├── token.rs        # OAuth2 토큰 발급·캐싱·자동갱신
│   ├── *_adapter.rs    # HttpClient의 포트 trait 구현
│   └── ...
└── error.rs            # 단일 SdkError 정의
```

- **`port/`** 는 "무엇을" 할 수 있는지를 정의(도메인 추상).
- **`adapter/`** 는 "어떻게" HTTP로 수행하는지를 구현(전송 세부사항).
- **`domain/models/`** 는 API 스펙의 요청·응답 구조체.

이 분리 덕분에 테스트에서는 mock 서버를 향해 `HttpClient`를 구성하기만 하면 됩니다.

## 품질

```bash
cargo fmt                                       # 포맷
cargo clippy --all-targets -- -D warnings       # 린트
cargo test                                      # 전체 회귀 테스트
```

## 라이선스

[Apache-2.0](LICENSE)
