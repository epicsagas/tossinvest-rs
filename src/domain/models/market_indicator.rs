//! 시장 지표(코스피·코스닥·S&P 500 등) 가격·캔들·투자자별 매매동향 모델.

use serde::{Deserialize, Serialize};

/// 시장 지표 현재가.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketIndicatorPriceResponse {
    pub symbol: String,
    #[serde(default)]
    pub timestamp: Option<String>,
    pub lastPrice: String,
}

/// 시장 지표 캔들 페이지 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketIndicatorCandlePageResponse {
    pub candles: Vec<MarketIndicatorCandle>,
    #[serde(default)]
    pub nextBefore: Option<String>,
}

/// 시장 지표 캔들.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketIndicatorCandle {
    pub timestamp: String,
    pub openPrice: String,
    pub highPrice: String,
    pub lowPrice: String,
    pub closePrice: String,
    pub volume: String,
}

/// 투자자별 매매동향 응답.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorTradingResponse {
    /// 다음 페이지 조회용 커서.
    #[serde(default)]
    pub nextUntil: Option<String>,
    pub records: Vec<InvestorTradingRecord>,
}

/// 투자자별 매매동향 기록.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorTradingRecord {
    /// 집계 기준일 (`YYYY-MM-DD`).
    pub date: String,
    /// 해당 기록의 마지막 갱신 시각 (RFC3339).
    pub updatedAt: String,
    pub individual: InvestorTradingAmount,
    /// 외국인 합계 (등록·미등록 외국인 포함).
    pub foreigner: InvestorTradingAmount,
    /// 기관 합계 (`buyAmount`/`sellAmount` 는 `breakdown` 7개 항목의 합).
    pub institution: InstitutionTradingAmount,
    /// 기타법인.
    pub otherCorporation: InvestorTradingAmount,
}

/// 투자자별 매매 금액.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestorTradingAmount {
    pub buyAmount: String,
    pub sellAmount: String,
}

/// 기관별 매매 금액 (세부 항목 포함).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionTradingAmount {
    pub buyAmount: String,
    pub sellAmount: String,
    pub breakdown: InstitutionTradingBreakdown,
}

/// 기관 세부 매매 분석.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstitutionTradingBreakdown {
    #[serde(default)]
    pub financialInvestment: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub insurance: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub trust: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub privateEquityFund: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub bank: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub otherFinancialInstitution: Option<InvestorTradingAmount>,
    #[serde(default)]
    pub pensionFund: Option<InvestorTradingAmount>,
}
