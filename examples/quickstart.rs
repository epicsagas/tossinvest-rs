//! README 퀵스타트 예제의 컴파일 보장용입니다.
//! `cargo build --examples` 가 통과하면 README 코드 샘플이 항상 빌드됨을 보장합니다.
//!
//! 실제 API 호출은 수행하지 않습니다(자격 증명 없음). 예제의 *형태*가 유효한지만 검증합니다.

use tossinvest_rs::v1::{HttpClient, MarketDataPort, TradingPort};

#[tokio::main]
async fn main() -> Result<(), tossinvest_rs::v1::SdkError> {
    // 1. 클라이언트 자격증명(client_id, client_secret) 으로 클라이언트를 초기화합니다.
    //    OAuth2 액세스 토큰이 자동으로 발급되어 `Authorization: Bearer` 로 주입됩니다.
    let client = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?;

    // 2. 시장 데이터 조회 (계좌 불필요).
    let _prices = client.get_prices(&["005930"]).await?;

    // 3. 주문·잔고 등 계좌 기반 API는 `with_account(accountSeq)` 로 기본 계좌를 지정합니다.
    let _ = HttpClient::new("YOUR_CLIENT_ID", "YOUR_CLIENT_SECRET")?
        .with_account(123456) // GET /accounts 로 얻은 accountSeq
        .get_buying_power(tossinvest_rs::v1::domain::models::Currency::Krw)
        .await?;

    Ok(())
}
