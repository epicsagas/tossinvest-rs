use thiserror::Error;

/// SDK에서 발생할 수 있는 단일 에러 타입입니다.
///
/// `HttpClient::new` 초기화와 모든 API 호출이 공통으로 반환하므로,
/// 호출자는 하나의 에러 타입만 다루면 됩니다.
#[derive(Error, Debug)]
pub enum SdkError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Invalid header value: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("API error: {status} {code} - {message}")]
    ApiError {
        status: reqwest::StatusCode,
        code: String,
        message: String,
        /// 토스증권 CS 문의 시 첨부를 권장하는 요청 식별자.
        /// OAuth2 토큰 엔드포인트 오류에는 존재하지 않습니다.
        request_id: Option<String>,
    },
    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl SdkError {
    /// 비-2xx 응답 본문에서 에러 정보를 추출해 `ApiError`를 만듭니다.
    ///
    /// 비즈니스 엔드포인트는 `{"error": {"code", "message", "requestId", ...}}` envelope을,
    /// OAuth2 토큰 엔드포인트는 `{"error": "...", "error_description": "..."}` 표준 포맷을 사용합니다.
    /// 본문이 JSON이 아니거나 알 수 없는 형태면 원본을 보존합니다(정보 손실 없음).
    pub(crate) fn from_error_response(status: reqwest::StatusCode, content: &str) -> Self {
        let value = match serde_json::from_str::<serde_json::Value>(content) {
            Ok(v) => v,
            Err(_) => {
                return SdkError::ApiError {
                    status,
                    code: "UNKNOWN".to_string(),
                    message: content.to_string(),
                    request_id: None,
                }
            }
        };

        // 비즈니스 envelope: {"error": {"code","message","requestId"}}
        if let Some(err) = value.get("error").cloned() {
            if let Some(obj) = err.as_object() {
                let code = obj
                    .get("code")
                    .and_then(|v| v.as_str())
                    .unwrap_or("UNKNOWN")
                    .to_string();
                let message = obj
                    .get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or(content)
                    .to_string();
                let request_id = obj
                    .get("requestId")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string());
                return SdkError::ApiError {
                    status,
                    code,
                    message,
                    request_id,
                };
            }
            // OAuth2 표준 포맷: {"error": "...", "error_description": "..."}
            if let Some(code) = err.as_str() {
                let message = value
                    .get("error_description")
                    .and_then(|v| v.as_str())
                    .unwrap_or(code)
                    .to_string();
                return SdkError::ApiError {
                    status,
                    code: code.to_string(),
                    message,
                    request_id: None,
                };
            }
        }

        SdkError::ApiError {
            status,
            code: "UNKNOWN".to_string(),
            message: content.to_string(),
            request_id: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::StatusCode;

    #[test]
    fn parses_business_error_envelope() {
        let body = r#"{"error":{"requestId":"01HXYZABCDEFG123456789","code":"order-not-found","message":"주문을 찾을 수 없습니다."}}"#;
        let err = SdkError::from_error_response(StatusCode::NOT_FOUND, body);
        match err {
            SdkError::ApiError {
                status,
                code,
                message,
                request_id,
            } => {
                assert_eq!(status, StatusCode::NOT_FOUND);
                assert_eq!(code, "order-not-found");
                assert_eq!(message, "주문을 찾을 수 없습니다.");
                assert_eq!(request_id.as_deref(), Some("01HXYZABCDEFG123456789"));
            }
            other => panic!("expected ApiError, got {other:?}"),
        }
    }

    #[test]
    fn parses_oauth2_error_format() {
        let body =
            r#"{"error":"invalid_client","error_description":"Client authentication failed."}"#;
        let err = SdkError::from_error_response(StatusCode::UNAUTHORIZED, body);
        match err {
            SdkError::ApiError {
                code,
                message,
                request_id,
                ..
            } => {
                assert_eq!(code, "invalid_client");
                assert_eq!(message, "Client authentication failed.");
                assert!(request_id.is_none());
            }
            other => panic!("expected ApiError, got {other:?}"),
        }
    }

    #[test]
    fn falls_back_on_non_json() {
        let err = SdkError::from_error_response(StatusCode::INTERNAL_SERVER_ERROR, "not json");
        match err {
            SdkError::ApiError { code, message, .. } => {
                assert_eq!(code, "UNKNOWN");
                assert_eq!(message, "not json");
            }
            other => panic!("expected ApiError, got {other:?}"),
        }
    }
}
