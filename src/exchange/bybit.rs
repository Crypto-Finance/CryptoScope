use async_trait::async_trait;
use reqwest::Client;
use tracing::{debug, info, warn};

use super::exchange_trait::Exchange;
use crate::error::{CryptoScopeError, Result};
use crate::models::{BybitApiResponse, Symbol};

const BYBIT_BASE_URL: &str = "https://api.bybit.com";
const INSTRUMENTS_ENDPOINT: &str = "/v5/market/instruments-info";

/// Bybit exchange client
pub struct BybitClient {
    client: Client,
    base_url: String,
}

impl BybitClient {
    /// Create a new Bybit client
    pub fn new() -> Self {
        Self {
            client: Client::new(),
            base_url: BYBIT_BASE_URL.to_string(),
        }
    }

    /// Create a new Bybit client with custom base URL (useful for testing)
    #[allow(dead_code)]
    pub fn with_base_url(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    /// Fetch a single page of instruments
    async fn fetch_page(&self, category: &str, cursor: &str) -> Result<BybitApiResponse> {
        let mut url = format!("{}{}", self.base_url, INSTRUMENTS_ENDPOINT);
        url.push_str(&format!("?category={}", category));

        if !cursor.is_empty() {
            url.push_str(&format!("&cursor={}", cursor));
        }

        debug!("Fetching: {}", url);

        let response = self.client.get(&url).send().await?;
        let api_response: BybitApiResponse = response.json().await?;

        if !api_response.is_success() {
            return Err(CryptoScopeError::ApiError {
                code: api_response.ret_code,
                message: api_response.ret_msg,
            });
        }

        Ok(api_response)
    }
}

#[async_trait]
impl Exchange for BybitClient {
    fn name(&self) -> &'static str {
        "bybit"
    }

    async fn fetch_instruments(&self, category: &str) -> Result<Vec<Symbol>> {
        info!("Fetching {} instruments from Bybit...", category);

        let mut all_symbols = Vec::new();
        let mut cursor = String::new();
        let mut page_count = 0;

        loop {
            page_count += 1;
            let response = self.fetch_page(category, &cursor).await?;

            let mut symbols = response.result.list;
            // Inject the category into each symbol since API doesn't include it
            for symbol in &mut symbols {
                if symbol.category.is_none() {
                    symbol.category = Some(category.to_string());
                }
            }
            let count = symbols.len();
            all_symbols.extend(symbols);

            debug!("Page {}: fetched {} symbols", page_count, count);

            // Check if there are more pages
            match response.result.next_page_cursor {
                Some(next_cursor) if !next_cursor.is_empty() => {
                    cursor = next_cursor;
                }
                _ => {
                    info!(
                        "Completed fetching {} symbols from {} pages for category '{}'",
                        all_symbols.len(),
                        page_count,
                        category
                    );
                    break;
                }
            }

            // Safety limit to prevent infinite loops
            if page_count >= 100 {
                warn!("Reached maximum page limit (100). Stopping pagination.");
                break;
            }
        }

        Ok(all_symbols)
    }
}

impl Default for BybitClient {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bybit_client_creation() {
        let client = BybitClient::new();
        assert_eq!(client.name(), "bybit");
    }
}
