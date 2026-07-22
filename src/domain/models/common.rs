//! 공통 응답 래퍼와 API 전반에 쓰이는 열거형.

use serde::{Deserialize, Serialize};

/// 성공 응답 envelope. `{"result": T}` 형태.
///
/// 모든 비즈니스 엔드포인트의 200 응답이 이 래퍼로 감싸져 있으며,
/// 어댑터는 역직렬화 후 `result` 만 추출해 호출자에게 반환합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub result: T,
}

/// 통화.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    Krw,
    Usd,
}

impl Currency {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Krw => "KRW",
            Self::Usd => "USD",
        }
    }
}

/// 시장 국가.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketCountry {
    Kr,
    Us,
}

impl MarketCountry {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Kr => "KR",
            Self::Us => "US",
        }
    }
}

/// 계좌 유형.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Brokerage,
    OverseasDerivatives,
    PensionSavings,
    ReshoringInvestment,
}

/// 주문 상태.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    PendingCancel,
    PendingReplace,
    PartialFilled,
    Filled,
    Canceled,
    Rejected,
    CancelRejected,
    ReplaceRejected,
    Replaced,
}

/// 주문 방향.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderSide {
    Buy,
    Sell,
}

/// 호가 유형.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderType {
    Limit,
    Market,
}

/// 체결 조건.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TimeInForce {
    /// 당일 주문 (또는 KRX 정규장).
    Day,
    /// 종가 주문 (US 애프터마켓 체결부).
    Cls,
    /// 시가 주문 (US 프리마켓 개시부).
    Opg,
}

/// 주문 목록 조회 상태 필터.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderListStatus {
    /// 미체결(접수/부분체결 포함).
    Open,
    /// 체결완료·취소·거절.
    Closed,
}

impl OrderListStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Open => "OPEN",
            Self::Closed => "CLOSED",
        }
    }
}

/// 캔들 간격.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CandleInterval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "1d")]
    OneDay,
}

impl CandleInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OneMinute => "1m",
            Self::OneDay => "1d",
        }
    }
}

/// 거래대금/거래량 순위 종류.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RankingType {
    #[serde(rename = "MARKET_TRADING_AMOUNT")]
    MarketTradingAmount,
    #[serde(rename = "MARKET_TRADING_VOLUME")]
    MarketTradingVolume,
    #[serde(rename = "TOP_GAINERS")]
    TopGainers,
    #[serde(rename = "TOP_LOSERS")]
    TopLosers,
    #[serde(rename = "TOSS_SECURITIES_TRADING_AMOUNT")]
    TossSecuritiesTradingAmount,
    #[serde(rename = "TOSS_SECURITIES_TRADING_VOLUME")]
    TossSecuritiesTradingVolume,
}

impl RankingType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::MarketTradingAmount => "MARKET_TRADING_AMOUNT",
            Self::MarketTradingVolume => "MARKET_TRADING_VOLUME",
            Self::TopGainers => "TOP_GAINERS",
            Self::TopLosers => "TOP_LOSERS",
            Self::TossSecuritiesTradingAmount => "TOSS_SECURITIES_TRADING_AMOUNT",
            Self::TossSecuritiesTradingVolume => "TOSS_SECURITIES_TRADING_VOLUME",
        }
    }
}

/// 순위 집계 기간.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RankingDuration {
    #[serde(rename = "realtime")]
    Realtime,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1mo")]
    OneMonth,
    #[serde(rename = "3mo")]
    ThreeMonths,
    #[serde(rename = "6mo")]
    SixMonths,
    #[serde(rename = "1y")]
    OneYear,
}

impl RankingDuration {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Realtime => "realtime",
            Self::OneDay => "1d",
            Self::OneWeek => "1w",
            Self::OneMonth => "1mo",
            Self::ThreeMonths => "3mo",
            Self::SixMonths => "6mo",
            Self::OneYear => "1y",
        }
    }
}

/// 통화별 금액. `krw` 는 필수, `usd` 는 미국 시장 데이터에만 존재합니다.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Price {
    pub krw: String,
    pub usd: Option<String>,
}
