use crate::adapter::http_client::HttpClient;
use crate::adapter::push_opt;
use crate::domain::models::{
    CandleInterval, CandlePageResponse, Currency, ExchangeRateResponse, InvestorTradingResponse,
    KrMarketCalendarResponse, MarketCountry, MarketIndicatorCandlePageResponse,
    MarketIndicatorPriceResponse, OrderbookResponse, PriceLimitResponse, PriceResponse,
    RankingDuration, RankingResponse, RankingType, StockInfo, StockWarning, Trade,
    UsMarketCalendarResponse,
};
use crate::error::SdkError;
use crate::port::MarketDataPort;
use async_trait::async_trait;

#[async_trait]
impl MarketDataPort for HttpClient {
    async fn get_orderbook(&self, symbol: &str) -> Result<OrderbookResponse, SdkError> {
        let query = vec![("symbol", symbol.to_string())];
        self.get("/api/v1/orderbook", &query, false).await
    }

    async fn get_prices(&self, symbols: &[&str]) -> Result<Vec<PriceResponse>, SdkError> {
        let query = vec![("symbols", symbols.join(","))];
        self.get("/api/v1/prices", &query, false).await
    }

    async fn get_trades(&self, symbol: &str, count: Option<u32>) -> Result<Vec<Trade>, SdkError> {
        let mut query = vec![("symbol", symbol.to_string())];
        push_opt(&mut query, "count", count);
        self.get("/api/v1/trades", &query, false).await
    }

    async fn get_price_limit(&self, symbol: &str) -> Result<PriceLimitResponse, SdkError> {
        let query = vec![("symbol", symbol.to_string())];
        self.get("/api/v1/price-limits", &query, false).await
    }

    async fn get_candles(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        before: Option<&str>,
        adjusted: Option<bool>,
    ) -> Result<CandlePageResponse, SdkError> {
        let mut query = vec![
            ("symbol", symbol.to_string()),
            ("interval", interval.as_str().to_string()),
        ];
        push_opt(&mut query, "count", count);
        push_opt(&mut query, "before", before);
        push_opt(&mut query, "adjusted", adjusted);
        self.get("/api/v1/candles", &query, false).await
    }

    async fn get_stocks(&self, symbols: &[&str]) -> Result<Vec<StockInfo>, SdkError> {
        let query = vec![("symbols", symbols.join(","))];
        self.get("/api/v1/stocks", &query, false).await
    }

    async fn get_stock_warnings(&self, symbol: &str) -> Result<Vec<StockWarning>, SdkError> {
        let path = format!("/api/v1/stocks/{symbol}/warnings");
        self.get(&path, &[], false).await
    }

    async fn get_exchange_rate(
        &self,
        date_time: Option<&str>,
        base_currency: Currency,
        quote_currency: Currency,
    ) -> Result<ExchangeRateResponse, SdkError> {
        let mut query = vec![
            ("baseCurrency", base_currency.as_str().to_string()),
            ("quoteCurrency", quote_currency.as_str().to_string()),
        ];
        push_opt(&mut query, "dateTime", date_time);
        self.get("/api/v1/exchange-rate", &query, false).await
    }

    async fn get_kr_market_calendar(
        &self,
        date: Option<&str>,
    ) -> Result<KrMarketCalendarResponse, SdkError> {
        let mut query: Vec<(&'static str, String)> = Vec::new();
        push_opt(&mut query, "date", date);
        self.get("/api/v1/market-calendar/KR", &query, false).await
    }

    async fn get_us_market_calendar(
        &self,
        date: Option<&str>,
    ) -> Result<UsMarketCalendarResponse, SdkError> {
        let mut query: Vec<(&'static str, String)> = Vec::new();
        push_opt(&mut query, "date", date);
        self.get("/api/v1/market-calendar/US", &query, false).await
    }

    async fn get_rankings(
        &self,
        ranking_type: RankingType,
        market_country: MarketCountry,
        duration: RankingDuration,
        exclude_investment_caution: Option<bool>,
        count: Option<u32>,
    ) -> Result<RankingResponse, SdkError> {
        let mut query = vec![
            ("type", ranking_type.as_str().to_string()),
            ("marketCountry", market_country.as_str().to_string()),
            ("duration", duration.as_str().to_string()),
        ];
        push_opt(
            &mut query,
            "excludeInvestmentCaution",
            exclude_investment_caution,
        );
        push_opt(&mut query, "count", count);
        self.get("/api/v1/rankings", &query, false).await
    }

    async fn get_market_indicator_prices(
        &self,
        symbols: &[&str],
    ) -> Result<Vec<MarketIndicatorPriceResponse>, SdkError> {
        let query = vec![("symbols", symbols.join(","))];
        self.get("/api/v1/market-indicators/prices", &query, false)
            .await
    }

    async fn get_market_indicator_candles(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        before: Option<&str>,
    ) -> Result<MarketIndicatorCandlePageResponse, SdkError> {
        let path = format!("/api/v1/market-indicators/{symbol}/candles");
        let mut query = vec![("interval", interval.as_str().to_string())];
        push_opt(&mut query, "count", count);
        push_opt(&mut query, "before", before);
        self.get(&path, &query, false).await
    }

    async fn get_market_indicator_investor_trading(
        &self,
        symbol: &str,
        interval: CandleInterval,
        count: Option<u32>,
        until: Option<&str>,
    ) -> Result<InvestorTradingResponse, SdkError> {
        let path = format!("/api/v1/market-indicators/{symbol}/investor-trading");
        let mut query = vec![("interval", interval.as_str().to_string())];
        push_opt(&mut query, "count", count);
        push_opt(&mut query, "until", until);
        self.get(&path, &query, false).await
    }
}
