use crate::models::{Statistics, Symbol};
use terminal_size::{Width, terminal_size};

/// Format and display statistics as human-readable text output.
///
/// Provides formatted console output including headers, statistics breakdowns,
/// and sample symbol listings with terminal width awareness.
pub struct TextFormatter;

impl TextFormatter {
    /// Format and print the full report
    ///
    /// Outputs a complete formatted report including header, statistics,
    /// and sample symbols to stdout.
    pub fn format(
        exchange_name: &str,
        categories: &[&str],
        symbols: &[Symbol],
        stats: &Statistics,
    ) {
        println!();
        Self::print_header(exchange_name, categories);
        println!();
        Self::print_statistics(stats);
        println!();
        Self::print_sample_symbols(symbols);
        println!();
    }

    /// Print the report header with exchange and category information
    fn print_header(exchange_name: &str, categories: &[&str]) {
        let title = format!(
            "=== CryptoScope: {} Perpetual Symbols ===",
            exchange_name.to_uppercase()
        );

        println!("{}", title);
        println!();
        println!("Exchange: {}", exchange_name.to_uppercase());
        println!("Categories: {}", categories.join(", "));
    }

    /// Print statistics breakdown by status, category, and contract type
    fn print_statistics(stats: &Statistics) {
        println!("📊 Statistics:");
        println!("  Total Symbols: {}", stats.total_count);
        println!();

        Self::print_status_breakdown(stats);
        println!();
        Self::print_category_breakdown(stats);
        println!();
        Self::print_contract_breakdown(stats);
    }

    /// Print status breakdown section
    fn print_status_breakdown(stats: &Statistics) {
        println!("  By Status:");
        // Sort statuses for consistent output
        let mut status_counts: Vec<_> = stats.by_status.iter().collect();
        status_counts.sort_by(|a, b| b.1.cmp(a.1)); // Sort by count descending

        for (status, count) in status_counts {
            println!("    {}: {}", status, count);
        }
    }

    /// Print category breakdown section
    fn print_category_breakdown(stats: &Statistics) {
        println!("  By Category:");
        for (category, count) in &stats.by_category {
            let description = match category.as_str() {
                "linear" => " (USDT Perpetual)",
                "inverse" => " (Inverse Perpetual)",
                _ => "",
            };
            println!("    {}{}: {}", category.to_uppercase(), description, count);
        }
    }

    /// Print contract type breakdown section
    fn print_contract_breakdown(stats: &Statistics) {
        println!("  By Contract Type:");
        let mut contract_counts: Vec<_> = stats.by_contract_type.iter().collect();
        contract_counts.sort_by(|a, b| b.1.cmp(a.1));
        for (contract_type, count) in contract_counts {
            println!("    {}: {}", contract_type, count);
        }
    }

    /// Print a sample of symbols (first 20) with line wrapping
    ///
    /// Formats symbol names as a comma-separated list that wraps
    /// based on the terminal width.
    fn print_sample_symbols(symbols: &[Symbol]) {
        println!("📋 Sample Symbols (first 20):");

        let sample: Vec<_> = symbols.iter().take(20).collect();
        let symbol_names: Vec<_> = sample.iter().map(|s| s.symbol.as_str()).collect();

        // Format as comma-separated list with line wrapping
        let mut line = String::from("  ");
        let width = Self::get_terminal_width();

        for (i, symbol) in symbol_names.iter().enumerate() {
            let suffix = if i < symbol_names.len() - 1 { ", " } else { "" };
            let entry = format!("{}{}", symbol, suffix);

            if line.len() + entry.len() > width - 2 {
                println!("{}", line);
                line = String::from("  ");
            }
            line.push_str(&entry);
        }

        if !line.trim().is_empty() {
            println!("{}", line);
        }

        if symbols.len() > 20 {
            println!("  ... and {} more", symbols.len() - 20);
        }
    }

    /// Get the current terminal width in characters
    ///
    /// Returns the terminal width if available, otherwise defaults to 80.
    fn get_terminal_width() -> usize {
        terminal_size()
            .map(|(Width(w), _)| w as usize)
            .unwrap_or(80)
    }
}

/// Filter symbols based on various criteria.
///
/// Provides methods to filter symbols by search term, status, or a combination
/// of multiple criteria.
pub struct SymbolFilter;

impl SymbolFilter {
    /// Filter symbols by search term (case-insensitive)
    ///
    /// Returns symbols whose name contains the search term.
    /// Returns all symbols if the search term is empty.
    pub fn by_search(symbols: &[Symbol], search_term: &str) -> Vec<Symbol> {
        if search_term.is_empty() {
            return symbols.to_vec();
        }

        let search_lower = search_term.to_lowercase();
        symbols
            .iter()
            .filter(|s| s.symbol.to_lowercase().contains(&search_lower))
            .cloned()
            .collect()
    }

    /// Filter symbols by status
    ///
    /// Returns symbols matching the given status (case-insensitive).
    /// Returns all symbols if the status filter is empty.
    pub fn by_status(symbols: &[Symbol], status: &str) -> Vec<Symbol> {
        if status.is_empty() {
            return symbols.to_vec();
        }

        symbols
            .iter()
            .filter(|s| s.status.to_string().eq_ignore_ascii_case(status))
            .cloned()
            .collect()
    }

    /// Apply multiple filters
    ///
    /// Applies search and status filters sequentially.
    /// Returns all symbols if no filters are provided.
    pub fn apply(symbols: &[Symbol], search: Option<&str>, status: Option<&str>) -> Vec<Symbol> {
        let mut filtered = symbols.to_vec();

        if let Some(search_term) = search {
            filtered = Self::by_search(&filtered, search_term);
        }

        if let Some(status_filter) = status {
            filtered = Self::by_status(&filtered, status_filter);
        }

        filtered
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::symbol::SymbolStatus;

    fn create_test_symbol(symbol: &str, status: SymbolStatus) -> Symbol {
        Symbol {
            symbol: symbol.to_string(),
            status,
            category: Some("linear".to_string()),
            contract_type: Some("Linear".to_string()),
            base_coin: Some("BTC".to_string()),
            quote_coin: Some("USDT".to_string()),
            launch_time: None,
            delivery_time: None,
            delivery_fee_rate: None,
        }
    }

    #[test]
    fn test_filter_by_search() {
        let symbols = vec![
            create_test_symbol("BTCUSDT", SymbolStatus::Trading),
            create_test_symbol("ETHUSDT", SymbolStatus::Trading),
            create_test_symbol("BTCUSD", SymbolStatus::Trading),
        ];

        let filtered = SymbolFilter::by_search(&symbols, "BTC");
        assert_eq!(filtered.len(), 2);
    }

    #[test]
    fn test_filter_by_status() {
        let symbols = vec![
            create_test_symbol("BTCUSDT", SymbolStatus::Trading),
            create_test_symbol("NEWUSDT", SymbolStatus::PreLaunch),
        ];

        let filtered = SymbolFilter::by_status(&symbols, "Trading");
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].symbol, "BTCUSDT");
    }
}
