use serde::{Deserialize, Serialize};

/// Symbol status enum based on Bybit API
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum SymbolStatus {
    Trading,
    PreLaunch,
    Delivered,
    Closed,
    Delisted,
    #[serde(other)]
    Unknown,
}

impl std::fmt::Display for SymbolStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SymbolStatus::Trading => write!(f, "Trading"),
            SymbolStatus::PreLaunch => write!(f, "PreLaunch"),
            SymbolStatus::Delivered => write!(f, "Delivered"),
            SymbolStatus::Closed => write!(f, "Closed"),
            SymbolStatus::Delisted => write!(f, "Delisted"),
            SymbolStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

/// Represents a cryptocurrency trading instrument/symbol.
///
/// Contains metadata about a trading pair including status, contract type,
/// and other trading parameters from the Bybit exchange.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Symbol {
    /// The symbol/ticker name (e.g., "BTCUSDT")
    pub symbol: String,
    /// Current trading status of the symbol
    #[serde(rename = "status")]
    pub status: SymbolStatus,
    /// Category of the instrument (e.g., "linear", "inverse")
    #[serde(rename = "category", default)]
    pub category: Option<String>,
    /// Type of contract (e.g., "Linear", "InversePerpetual")
    #[serde(rename = "contractType", default)]
    pub contract_type: Option<String>,
    /// Base currency code (e.g., "BTC")
    #[serde(rename = "baseCoin", default)]
    pub base_coin: Option<String>,
    /// Quote currency code (e.g., "USDT")
    #[serde(rename = "quoteCoin", default)]
    pub quote_coin: Option<String>,
    /// Unix timestamp when the symbol was launched
    #[serde(rename = "launchTime", default)]
    pub launch_time: Option<String>,
    /// Unix timestamp for delivery (futures contracts)
    #[serde(rename = "deliveryTime", default)]
    pub delivery_time: Option<String>,
    /// Delivery fee rate for futures contracts
    #[serde(rename = "deliveryFeeRate", default)]
    pub delivery_fee_rate: Option<String>,
}

impl Symbol {
    /// Get category with fallback to empty string
    ///
    /// Returns the instrument category (e.g., "linear", "inverse") or "unknown"
    /// if not specified.
    pub fn category(&self) -> &str {
        self.category.as_deref().unwrap_or("unknown")
    }

    /// Get contract type with fallback
    ///
    /// Returns the contract type (e.g., "Linear", "InversePerpetual") or "Unknown"
    /// if not specified.
    pub fn contract_type(&self) -> &str {
        self.contract_type.as_deref().unwrap_or("Unknown")
    }

    /// Get base coin with fallback
    ///
    /// Returns the base currency code (e.g., "BTC") or "UNKNOWN" if not specified.
    pub fn base_coin(&self) -> &str {
        self.base_coin.as_deref().unwrap_or("UNKNOWN")
    }

    /// Get quote coin with fallback
    ///
    /// Returns the quote currency code (e.g., "USDT") or "UNKNOWN" if not specified.
    pub fn quote_coin(&self) -> &str {
        self.quote_coin.as_deref().unwrap_or("UNKNOWN")
    }
}

impl Symbol {
    /// Check if symbol is currently trading
    ///
    /// Returns `true` if the symbol status is `Trading`, indicating active trading.
    #[allow(dead_code)]
    pub fn is_trading(&self) -> bool {
        self.status == SymbolStatus::Trading
    }

    /// Get a short description of the contract type
    ///
    /// Returns a human-readable description of the contract type
    /// (e.g., "USDT Perpetual" for Linear contracts).
    #[allow(dead_code)]
    pub fn contract_description(&self) -> &str {
        match self.contract_type() {
            "Linear" => "USDT Perpetual",
            "InversePerpetual" => "Inverse Perpetual",
            "InverseFutures" => "Inverse Futures",
            other => other,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_trading_status() {
        let json = r#""Trading""#;
        let status: SymbolStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, SymbolStatus::Trading);
    }

    #[test]
    fn test_parse_unknown_status() {
        let json = r#""SomeNewStatus""#;
        let status: SymbolStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, SymbolStatus::Unknown);
    }
}
