use super::symbol::{Symbol, SymbolStatus};
use std::collections::HashMap;

/// Aggregated statistics about fetched symbols.
///
/// Contains counts of symbols organized by status, category, and contract type
/// for analysis and reporting purposes.
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    /// Total number of symbols in the dataset
    pub total_count: usize,
    /// Count of symbols grouped by their trading status
    pub by_status: HashMap<SymbolStatus, usize>,
    /// Count of symbols grouped by category (linear, inverse, etc.)
    pub by_category: HashMap<String, usize>,
    /// Count of symbols grouped by contract type
    pub by_contract_type: HashMap<String, usize>,
}

impl Statistics {
    /// Create statistics from a list of symbols
    ///
    /// Aggregates symbol counts by status, category, and contract type
    /// for analysis and reporting.
    pub fn from_symbols(symbols: &[Symbol]) -> Self {
        let mut by_status = HashMap::new();
        let mut by_category = HashMap::new();
        let mut by_contract_type = HashMap::new();

        for symbol in symbols {
            *by_status.entry(symbol.status.clone()).or_insert(0) += 1;
            *by_category
                .entry(symbol.category().to_string())
                .or_insert(0) += 1;
            *by_contract_type
                .entry(symbol.contract_type().to_string())
                .or_insert(0) += 1;
        }

        Self {
            total_count: symbols.len(),
            by_status,
            by_category,
            by_contract_type,
        }
    }

    /// Get count for a specific status
    ///
    /// Returns the number of symbols with the given status, or 0 if none.
    #[allow(dead_code)]
    pub fn count_by_status(&self, status: &SymbolStatus) -> usize {
        *self.by_status.get(status).unwrap_or(&0)
    }

    /// Get count for a specific category
    ///
    /// Returns the number of symbols in the given category, or 0 if none.
    #[allow(dead_code)]
    pub fn count_by_category(&self, category: &str) -> usize {
        *self.by_category.get(category).unwrap_or(&0)
    }

    /// Get the most common status
    ///
    /// Returns the status with the highest count and its count,
    /// or `None` if there are no symbols.
    #[allow(dead_code)]
    pub fn most_common_status(&self) -> Option<(&SymbolStatus, usize)> {
        self.by_status
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(status, count)| (status, *count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_symbol(symbol: &str, status: SymbolStatus, category: &str) -> Symbol {
        Symbol {
            symbol: symbol.to_string(),
            status,
            category: Some(category.to_string()),
            contract_type: if category == "linear" {
                Some("Linear".to_string())
            } else {
                Some("InversePerpetual".to_string())
            },
            base_coin: Some("BTC".to_string()),
            quote_coin: Some("USDT".to_string()),
            launch_time: None,
            delivery_time: None,
            delivery_fee_rate: None,
        }
    }

    #[test]
    fn test_statistics_aggregation() {
        let symbols = vec![
            create_test_symbol("BTCUSDT", SymbolStatus::Trading, "linear"),
            create_test_symbol("ETHUSDT", SymbolStatus::Trading, "linear"),
            create_test_symbol("BTCUSD", SymbolStatus::Trading, "inverse"),
            create_test_symbol("NEWUSDT", SymbolStatus::PreLaunch, "linear"),
        ];

        let stats = Statistics::from_symbols(&symbols);

        assert_eq!(stats.total_count, 4);
        assert_eq!(stats.count_by_status(&SymbolStatus::Trading), 3);
        assert_eq!(stats.count_by_status(&SymbolStatus::PreLaunch), 1);
        assert_eq!(stats.count_by_category("linear"), 3);
        assert_eq!(stats.count_by_category("inverse"), 1);
    }
}
