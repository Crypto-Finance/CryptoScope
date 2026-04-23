use super::symbol::Symbol;
use std::collections::HashMap;

/// Aggregated statistics about fetched symbols.
///
/// Contains counts of symbols organized by category and contract type
/// for analysis and reporting purposes.
#[derive(Debug, Clone, Default)]
pub struct Statistics {
    /// Total number of symbols in the dataset
    pub total_count: usize,
    /// Count of symbols grouped by category (linear, inverse, etc.)
    pub by_category: HashMap<String, usize>,
    /// Count of symbols grouped by contract type
    pub by_contract_type: HashMap<String, usize>,
}

impl Statistics {
    /// Create statistics from a list of symbols
    ///
    /// Aggregates symbol counts by category and contract type
    /// for analysis and reporting.
    pub fn from_symbols(symbols: &[Symbol]) -> Self {
        let mut by_category = HashMap::new();
        let mut by_contract_type = HashMap::new();

        for symbol in symbols {
            *by_category
                .entry(symbol.category().to_string())
                .or_insert(0) += 1;
            *by_contract_type
                .entry(symbol.contract_type().to_string())
                .or_insert(0) += 1;
        }

        Self {
            total_count: symbols.len(),
            by_category,
            by_contract_type,
        }
    }

    /// Get count for a specific category
    ///
    /// Returns the number of symbols in the given category, or 0 if none.
    #[allow(dead_code)]
    pub fn count_by_category(&self, category: &str) -> usize {
        *self.by_category.get(category).unwrap_or(&0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_symbol(symbol: &str, category: &str) -> Symbol {
        Symbol {
            symbol: symbol.to_string(),
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
            create_test_symbol("BTCUSDT", "linear"),
            create_test_symbol("ETHUSDT", "linear"),
            create_test_symbol("BTCUSD", "inverse"),
        ];

        let stats = Statistics::from_symbols(&symbols);

        assert_eq!(stats.total_count, 3);
        assert_eq!(stats.count_by_category("linear"), 2);
        assert_eq!(stats.count_by_category("inverse"), 1);
    }
}
