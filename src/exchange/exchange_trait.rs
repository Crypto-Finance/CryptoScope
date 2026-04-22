use crate::error::Result;
use crate::models::Symbol;
use async_trait::async_trait;

/// Trait that all exchange clients must implement
#[async_trait]
pub trait Exchange: Send + Sync {
    /// Get the exchange name
    fn name(&self) -> &'static str;

    /// Fetch instruments for a specific category
    /// This method handles pagination internally
    async fn fetch_instruments(&self, category: &str) -> Result<Vec<Symbol>>;

    /// Fetch instruments for multiple categories
    #[allow(dead_code)]
    async fn fetch_all_instruments(&self, categories: &[&str]) -> Result<Vec<Symbol>> {
        let mut all_symbols = Vec::new();
        for category in categories {
            let symbols = self.fetch_instruments(category).await?;
            all_symbols.extend(symbols);
        }
        Ok(all_symbols)
    }
}
