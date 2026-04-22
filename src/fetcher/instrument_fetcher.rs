use crate::error::Result;
use crate::exchange::Exchange;
use crate::models::Symbol;
use tracing::info;

/// Fetches instruments from an exchange with pagination support
pub struct InstrumentFetcher;

impl InstrumentFetcher {
    /// Fetch instruments for a single category
    #[allow(dead_code)]
    pub async fn fetch_category(exchange: &dyn Exchange, category: &str) -> Result<Vec<Symbol>> {
        exchange.fetch_instruments(category).await
    }

    /// Fetch instruments for multiple categories
    pub async fn fetch_categories(
        exchange: &dyn Exchange,
        categories: &[&str],
    ) -> Result<Vec<Symbol>> {
        let mut all_symbols = Vec::new();

        for category in categories {
            info!("Fetching category: {}", category);
            let symbols = exchange.fetch_instruments(category).await?;
            info!("  → Got {} symbols from {}", symbols.len(), category);
            all_symbols.extend(symbols);
        }

        Ok(all_symbols)
    }
}

#[cfg(test)]
mod tests {
    // Note: parse_categories functionality has been moved to Cli::get_categories()
    // Tests for category parsing are now in src/cli.rs
}
