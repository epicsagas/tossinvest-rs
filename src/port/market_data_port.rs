use crate::domain::models::{
    CandleInterval, CandlePageResponse, Currency, ExchangeRateResponse, InvestorTradingResponse,
    KrMarketCalendarResponse, MarketCountry, MarketIndicatorCandlePageResponse,
    MarketIndicatorPriceResponse, OrderbookResponse, PriceLimitResponse, PriceResponse,
    RankingDuration, RankingResponse, RankingType, StockInfo, StockWarning, Trade,
    UsMarketCalendarResponse,
};
use crate::error::SdkError;
use async_trait::async_trait;

/// 시장 데이터 포트 — 시세·호가·체결·캔들·종목정보·순위·환율·시장캘린더·투자자매매동향 등
/// 조회 전용 엔드포인트. 계좌(`AccountSeq`) 가 필요 없습니다.
#[async_trait]
pub trait MarketDataPort {
    /// 호가창 조회 (`GET /api/v1/orderbook`).
    async fn get_orderbook(&self, symbol: &str) -> Result<OrderbookResponse, SdkError>;
    /// 현재가 조회 (`GET /api/v1/prices`). `symbols` 는 종목 심볼 목록.
    async fn get_prices(&self, symbols: &[&str]) -> Result<Vec<PriceResponse>, SdkError>;
    /// 체결 내역 조회 (`GET /api/v1/trades`). `count` 기본값 50(최대 50).
    async fn get_trades(&self, symbol: &str, count: Option<u32>) -> Result<Vec<Trade>, SdkError>;
    /// 가격 제한폭 조회 (`GET /api/v1/price-limits`).
    async fn get_price_limit(&self, symbol: &str) -> Result<PriceLimitResponse, SdkError>;
    /// 캔들 조회 (`GET /api/v1/candles`).
    async fn get_candles(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        before: Option<&str>,
        adjusted: Option<bool>,
    ) -> Result<CandlePageResponse, SdkError>;
    /// 종목 정보 조회 (`GET /api/v1/stocks`).
    async fn get_stocks(&self, symbols: &[&str]) -> Result<Vec<StockInfo>, SdkError>;
    /// 투자유의 종목 경고 조회 (`GET /api/v1/stocks/{symbol}/warnings`).
    async fn get_stock_warnings(&self, symbol: &str) -> Result<Vec<StockWarning>, SdkError>;
    /// 환율 조회 (`GET /api/v1/exchange-rate`).
    async fn get_exchange_rate(
        &self,
        date_time: Option<&str>,
        base_currency: Currency,
        quote_currency: Currency,
    ) -> Result<ExchangeRateResponse, SdkError>;
    /// 한국 시장 영업일 캘린더 (`GET /api/v1/market-calendar/KR`).
    async fn get_kr_market_calendar(
        &self,
        date: Option<&str>,
    ) -> Result<KrMarketCalendarResponse, SdkError>;
    /// 미국 시장 영업일 캘린더 (`GET /api/v1/market-calendar/US`).
    async fn get_us_market_calendar(
        &self,
        date: Option<&str>,
    ) -> Result<UsMarketCalendarResponse, SdkError>;
    /// 순위 조회 (`GET /api/v1/rankings`).
    async fn get_rankings(
        &self,
        ranking_type: RankingType,
        market_country: MarketCountry,
        duration: RankingDuration,
        exclude_investment_caution: Option<bool>,
        count: Option<u32>,
    ) -> Result<RankingResponse, SdkError>;
    /// 시장 지표 현재가 (`GET /api/v1/market-indicators/prices`).
    async fn get_market_indicator_prices(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<MarketIndicatorPriceResponse>, SdkError>;
    /// 시장 지표 캔들 (`GET /api/v1/market-indicators/{symbol}/candles`).
    async fn get_market_indicator_candles(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        before: Option<&str>,
    ) -> Result<MarketIndicatorCandlePageResponse, SdkError>;
    /// 시장 지표 투자자별 매매동향 (`GET /api/v1/market-indicators/{symbol}/investor-trading`).
    async fn get_market_indicator_investor_trading(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        until: Option<&str>,
    ) -> Result<InvestorTradingResponse, SdkError>;
}
