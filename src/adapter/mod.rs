pub mod account_adapter;
pub mod auth_adapter;
pub mod conditional_order_adapter;
pub mod http_client;
pub mod market_data_adapter;
pub mod token;
pub mod trading_adapter;

pub use http_client::*;

/// 선택적 쿼리 파라미터를 빌더에 추가합니다.
pub(crate) fn push_opt<T: std::string::ToString>(
    query: &mut Vec<(&'static str, String)>,
    key: &'static str,
    value: Option<T>,
) {
    if let Some(value) = value {
        query.push((key, value.to_string()));
    }
}
