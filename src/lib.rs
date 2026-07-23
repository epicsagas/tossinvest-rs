//! # Toss Securities (토스증권) Rust SDK (Unofficial)
//!
//! ⚠️ **비공식 (Unofficial) Rust SDK**
//! 본 라이브러리는 개인이 개발한 비공식 Rust SDK입니다.
//! 토스증권(Toss Securities)의 공식 지원이나 관리를 받지 않는 독립적인 오픈소스 프로젝트입니다.
//! 공식 정보 및 문서는 [토스증권 Open API 문서](https://openapi.tossinvest.com/)를 참고하세요.

// 응답 모델 필드는 JSON 키와 1:1로 일치(camelCase)시켜 역직렬화를 직관적으로 유지합니다.
#![allow(non_snake_case)]

pub mod adapter;
pub mod domain;
pub mod error;
pub mod port;

/// 편의 재노출 프렐류드.
///
/// `use tossinvest_rs::prelude::*;` 로 클라이언트·포트 trait·에러 타입을 한 번에 가져옵니다.
/// 도메인 모델은 [`crate::domain::models`] (`tossinvest_rs::domain::models`) 경로로 접근하세요.
pub mod prelude {
    pub use crate::adapter::http_client::HttpClient;
    pub use crate::error::SdkError;
    pub use crate::port::{
        AccountPort, AuthPort, ConditionalOrderPort, MarketDataPort, TradingPort,
    };
}

pub mod v1 {
    pub use crate::adapter;
    pub use crate::domain;
    pub use crate::error;
    pub use crate::port;

    pub use crate::adapter::http_client::HttpClient;
    pub use crate::error::SdkError;
    pub use crate::port::*;
}

pub use v1::*;
