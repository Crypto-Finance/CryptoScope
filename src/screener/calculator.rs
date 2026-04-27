use crate::models::{ContractType, PriceChange, Ticker};
use std::collections::HashMap;

pub fn calculate_all(
    open_prices: Vec<(String, f64)>,
    current_tickers: Vec<Ticker>,
) -> Vec<PriceChange> {
    let open_price_map: HashMap<&str, f64> = open_prices
        .iter()
        .map(|(symbol, price)| (symbol.as_str(), *price))
        .collect();

    current_tickers
        .into_iter()
        .filter_map(|ticker| {
            // Try DB open price first, fall back to prev_price_24h
            let open_price = open_price_map.get(ticker.symbol.as_str()).copied().or(
                if ticker.prev_price_24h > 0.0 {
                    Some(ticker.prev_price_24h)
                } else {
                    None
                },
            )?;

            let change_value = ticker.last_price - open_price;
            let change_percent = if open_price != 0.0 {
                (change_value / open_price) * 100.0
            } else {
                0.0
            };

            let contract_type = infer_contract_type_from_category(&ticker.category);

            Some(PriceChange {
                symbol: ticker.symbol,
                category: ticker.category,
                contract_type,
                open_price,
                current_price: ticker.last_price,
                change_value,
                change_percent,
                volume_24h: ticker.volume_24h,
            })
        })
        .collect()
}

/// Fallback: infer contract type from category string when symbol map lookup fails.
fn infer_contract_type_from_category(category: &str) -> ContractType {
    match category {
        "linear" => ContractType::LinearPerpetual,
        "inverse" => ContractType::InversePerpetual,
        _ => ContractType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_ticker(symbol: &str, last_price: f64, volume_24h: f64) -> Ticker {
        Ticker {
            symbol: symbol.to_string(),
            category: "linear".to_string(),
            last_price,
            prev_price_24h: 0.0,
            volume_24h,
        }
    }

    #[test]
    fn test_calculate_all_basic() {
        let open_prices = vec![
            ("BTCUSDT".to_string(), 50000.0),
            ("ETHUSDT".to_string(), 3000.0),
        ];
        let tickers = vec![
            create_test_ticker("BTCUSDT", 51000.0, 1000.0),
            create_test_ticker("ETHUSDT", 3100.0, 500.0),
        ];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 2);

        let btc = results.iter().find(|r| r.symbol == "BTCUSDT").unwrap();
        assert_eq!(btc.open_price, 50000.0);
        assert_eq!(btc.current_price, 51000.0);
        assert_eq!(btc.change_value, 1000.0);
        assert!((btc.change_percent - 2.0).abs() < 0.01);
        assert_eq!(btc.category, "linear");

        let eth = results.iter().find(|r| r.symbol == "ETHUSDT").unwrap();
        assert_eq!(eth.open_price, 3000.0);
        assert_eq!(eth.current_price, 3100.0);
        assert_eq!(eth.change_value, 100.0);
        assert!((eth.change_percent - 3.333).abs() < 0.1);
    }

    #[test]
    fn test_calculate_all_filters_missing_symbols() {
        let open_prices = vec![("BTCUSDT".to_string(), 50000.0)];
        let tickers = vec![
            create_test_ticker("BTCUSDT", 51000.0, 1000.0),
            create_test_ticker("ETHUSDT", 3100.0, 500.0), // No open price
        ];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].symbol, "BTCUSDT");
    }

    #[test]
    fn test_calculate_all_handles_zero_open_price() {
        let open_prices = vec![("BTCUSDT".to_string(), 0.0)];
        let tickers = vec![create_test_ticker("BTCUSDT", 51000.0, 1000.0)];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].change_percent, 0.0); // Should not divide by zero
    }

    #[test]
    fn test_calculate_all_negative_change() {
        let open_prices = vec![("BTCUSDT".to_string(), 50000.0)];
        let tickers = vec![create_test_ticker("BTCUSDT", 49000.0, 1000.0)];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].change_value, -1000.0);
        assert!((results[0].change_percent - (-2.0)).abs() < 0.01);
    }

    #[test]
    fn test_calculate_all_empty_inputs() {
        let results = calculate_all(vec![], vec![]);
        assert!(results.is_empty());

        let results = calculate_all(vec![("BTCUSDT".to_string(), 50000.0)], vec![]);
        assert!(results.is_empty());

        let tickers = vec![create_test_ticker("BTCUSDT", 51000.0, 1000.0)];
        let results = calculate_all(vec![], tickers);
        assert!(results.is_empty());
    }

    #[test]
    fn test_calculate_all_preserves_category() {
        let open_prices = vec![("BTCUSDT".to_string(), 50000.0)];
        let tickers = vec![create_test_ticker("BTCUSDT", 51000.0, 1000.0)];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].category, "linear");
        assert!(results[0].is_derivative());
    }

    #[test]
    fn test_calculate_all_fallback_to_prev_price_24h() {
        // No open prices in DB, but ticker has prev_price_24h
        let open_prices: Vec<(String, f64)> = vec![];
        let mut ticker = create_test_ticker("BTCUSDT", 51000.0, 1000.0);
        ticker.prev_price_24h = 50000.0;
        let tickers = vec![ticker];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        let btc = &results[0];
        assert_eq!(btc.open_price, 50000.0); // Used prev_price_24h as fallback
        assert_eq!(btc.current_price, 51000.0);
        assert_eq!(btc.change_value, 1000.0);
        assert!((btc.change_percent - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_calculate_all_db_open_price_takes_precedence_over_prev_price() {
        // Both DB open price and prev_price_24h available — DB should win
        let open_prices = vec![("BTCUSDT".to_string(), 49000.0)];
        let mut ticker = create_test_ticker("BTCUSDT", 51000.0, 1000.0);
        ticker.prev_price_24h = 50000.0;
        let tickers = vec![ticker];

        let results = calculate_all(open_prices, tickers);

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].open_price, 49000.0); // DB price, not prev_price_24h
    }

    #[test]
    fn test_calculate_all_still_drops_when_no_data_at_all() {
        // No DB open price AND prev_price_24h is 0 — should still be dropped
        let open_prices: Vec<(String, f64)> = vec![];
        let tickers = vec![create_test_ticker("BTCUSDT", 51000.0, 1000.0)];

        let results = calculate_all(open_prices, tickers);

        assert!(results.is_empty());
    }
}
