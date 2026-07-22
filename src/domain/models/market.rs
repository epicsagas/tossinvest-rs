//! 시세·호가·체결·캔들·종목정보·가격제한·환율 등 시장 데이터 모델.

use serde::{Deserialize, Serialize};

use super::common::Currency;

/// 호가창 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookResponse {
    #[serde(default)]
    pub timestamp: Option<String>,
    pub currency: Currency,
    pub asks: Vec<OrderbookEntry>,
    pub bids: Vec<OrderbookEntry>,
}

/// 호가 항목.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderbookEntry {
    pub price: String,
    pub volume: String,
}

/// 단일 종목 현재가 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceResponse {
    pub symbol: String,
    #[serde(default)]
    pub timestamp: Option<String>,
    pub lastPrice: String,
    pub currency: Currency,
}

/// 체결 내역.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trade {
    pub price: String,
    pub volume: String,
    pub timestamp: String,
    pub currency: Currency,
}

/// 캔들 (OHLCV).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candle {
    pub timestamp: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub closePrice: String,
    pub volume: String,
    pub currency: Currency,
}

/// 캔들 페이지 응답. 시간 역순 정렬, `nextBefore` 로 다음 페이지 조회.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandlePageResponse {
    pub candles: Vec<Candle>,
    #[serde(default)]
    pub nextBefore: Option<String>,
}

/// 가격 제한폭 응답 (상한가/하한가).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceLimitResponse {
    pub timestamp: String,
    #[serde(default)]
    pub upperLimitPrice: Option<String>,
    #[serde(default)]
    pub lowerLimitPrice: Option<String>,
    pub currency: Currency,
}

/// 종목 기본 정보.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockInfo {
    pub symbol: String,
    pub name: String,
    pub englishName: String,
    pub isinCode: String,
    /// 시장 분류 코드 (예: `KRX`, `NASDAQ`).
    pub market: String,
    /// 증권 종류 (예: `STOCK`, `ETF`).
    pub securityType: String,
    pub isCommonShare: bool,
    /// 종목 상태 (예: `ACTIVE`, `DELISTED`).
    pub status: String,
    pub currency: Currency,
    #[serde(default)]
    pub listDate: Option<String>,
    #[serde(default)]
    pub delistDate: Option<String>,
    pub sharesOutstanding: String,
    #[serde(default)]
    pub leverageFactor: Option<String>,
    #[serde(default)]
    pub koreanMarketDetail: Option<KrMarketDetail>,
}

/// 한국 시장 상세 정보.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrMarketDetail {
    pub liquidationTrading: bool,
    pub nxtSupported: bool,
    pub krxTradingSuspended: bool,
    #[serde(default)]
    pub nxtTradingSuspended: Option<bool>,
}

/// 투자유의 종목 경고.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockWarning {
    /// 경고 유형 (예: `MANAGEMENT`, `INVESTMENT_CAUTION`).
    pub warningType: String,
    #[serde(default)]
    pub exchange: Option<String>,
    #[serde(default)]
    pub startDate: Option<String>,
    #[serde(default)]
    pub endDate: Option<String>,
}

/// 환율 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRateResponse {
    pub baseCurrency: Currency,
    pub quoteCurrency: Currency,
    pub rate: String,
    pub midRate: String,
    pub basisPoint: String,
    /// 환율 변동 유형 (예: `UP`, `DOWN`, `FLAT`).
    pub rateChangeType: String,
    pub validFrom: String,
    pub validUntil: String,
}
