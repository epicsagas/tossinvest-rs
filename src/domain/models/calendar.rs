//! 한국·미국 시장 영업일 캘린더와 거래 세션 시간 모델.

use serde::{Deserialize, Serialize};

/// 한국 시장 영업일 캘린더 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrMarketCalendarResponse {
    pub today: KrMarketDay,
    pub previousBusinessDay: KrMarketDay,
    pub nextBusinessDay: KrMarketDay,
}

/// 한국 시장 영업일.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KrMarketDay {
    /// 영업일 (KST, `YYYY-MM-DD`).
    pub date: String,
    /// 통합(KRX+NXT) 거래 가능 시간. 둘 다 휴장이면 `null`.
    #[serde(default)]
    pub integrated: Option<IntegratedHour>,
}

/// 통합 거래 가능 시간 (특수장 제외).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedHour {
    #[serde(default)]
    pub preMarket: Option<PreMarketSession>,
    #[serde(default)]
    pub regularMarket: Option<RegularMarketSession>,
    #[serde(default)]
    pub afterMarket: Option<AfterMarketSession>,
}

/// 프리마켓 세션 (NXT 접속매매).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreMarketSession {
    pub startTime: String,
    #[serde(default)]
    pub singlePriceAuctionStartTime: Option<String>,
    pub endTime: String,
}

/// 정규장 세션 (KRX·NXT 합집합).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegularMarketSession {
    pub startTime: String,
    #[serde(default)]
    pub singlePriceAuctionStartTime: Option<String>,
    pub endTime: String,
}

/// 애프터마켓 세션 (NXT).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AfterMarketSession {
    pub startTime: String,
    #[serde(default)]
    pub singlePriceAuctionEndTime: Option<String>,
    pub endTime: String,
}

/// 미국 시장 영업일 캘린더 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsMarketCalendarResponse {
    pub today: UsMarketDay,
    pub previousBusinessDay: UsMarketDay,
    pub nextBusinessDay: UsMarketDay,
}

/// 미국 시장 영업일.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsMarketDay {
    /// 영업일 (미국 현지, `YYYY-MM-DD`).
    pub date: String,
    #[serde(default)]
    pub dayMarket: Option<UsDayMarketSession>,
    #[serde(default)]
    pub preMarket: Option<UsPreMarketSession>,
    #[serde(default)]
    pub regularMarket: Option<UsRegularMarketSession>,
    #[serde(default)]
    pub afterMarket: Option<UsAfterMarketSession>,
}

/// 미국 데이마켓 세션 (토스증권).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsDayMarketSession {
    pub startTime: String,
    pub endTime: String,
}

/// 미국 프리마켓 세션.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsPreMarketSession {
    pub startTime: String,
    pub endTime: String,
}

/// 미국 정규장 세션.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsRegularMarketSession {
    pub startTime: String,
    pub endTime: String,
}

/// 미국 애프터마켓 세션.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsAfterMarketSession {
    pub startTime: String,
    pub endTime: String,
}
