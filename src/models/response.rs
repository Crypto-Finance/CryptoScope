use super::symbol::Symbol;
use serde::{Deserialize, Serialize};

/// Bybit API response structure for instruments-info endpoint
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BybitApiResponse {
    #[serde(rename = "retCode")]
    pub ret_code: i32,
    #[serde(rename = "retMsg")]
    pub ret_msg: String,
    pub result: BybitInstrumentsResult,
    #[serde(rename = "retExtInfo", default)]
    pub ret_ext_info: serde_json::Value,
    pub time: i64,
}

/// Result section containing the instruments list and pagination info.
///
/// This struct wraps the list of symbols returned by the Bybit API
/// along with category information and pagination cursor for fetching
/// additional pages of results.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BybitInstrumentsResult {
    /// The category of instruments (e.g., "linear", "inverse")
    #[serde(rename = "category")]
    pub category: String,
    /// List of trading symbols/instruments in this category
    pub list: Vec<Symbol>,
    /// Cursor for fetching the next page of results (if available)
    #[serde(rename = "nextPageCursor", default)]
    pub next_page_cursor: Option<String>,
}

impl BybitApiResponse {
    /// Check if API call was successful
    ///
    /// Returns `true` if the response code is 0, indicating success.
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }

    /// Get error message if request failed
    ///
    /// Returns `None` if the request was successful, otherwise returns
    /// a formatted error message with the response code.
    #[allow(dead_code)]
    pub fn error_message(&self) -> Option<String> {
        if self.is_success() {
            None
        } else {
            Some(format!("{} (code: {})", self.ret_msg, self.ret_code))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sample_response() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {
                "category": "linear",
                "list": [
                    {
                        "symbol": "BTCUSDT",
                        "status": "Trading",
                        "category": "linear",
                        "contractType": "Linear",
                        "baseCoin": "BTC",
                        "quoteCoin": "USDT"
                    }
                ],
                "nextPageCursor": ""
            },
            "time": 1234567890
        }"#;

        let response: BybitApiResponse = serde_json::from_str(json).unwrap();
        assert!(response.is_success());
        assert_eq!(response.result.list.len(), 1);
        assert_eq!(response.result.list[0].symbol, "BTCUSDT");
    }
}
