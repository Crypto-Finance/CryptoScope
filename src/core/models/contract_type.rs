/// Represents the type of a cryptocurrency derivatives contract.
///
/// Used to categorize symbols by their contract structure:
/// - Linear: Settled in quote currency (e.g., USDT)
/// - Inverse: Settled in base currency (e.g., BTC)
/// - Perpetual: No expiry date
/// - Futures: Fixed expiry/delivery date
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde::Serialize,
    serde::Deserialize,
    utoipa::ToSchema,
    ts_rs::TS,
)]
#[ts(export)]
pub enum ContractType {
    LinearPerpetual,
    LinearFutures,
    InversePerpetual,
    InverseFutures,
    Unknown,
}

impl ContractType {
    /// Parse a contract type from a string.
    ///
    /// Matches against common API response formats (case-insensitive).
    /// Returns `ContractType::Unknown` for unrecognized values.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "linearperpetual" | "linear_perpetual" => ContractType::LinearPerpetual,
            "linearfutures" | "linear_futures" => ContractType::LinearFutures,
            "inverseperpetual" | "inverse_perpetual" => ContractType::InversePerpetual,
            "inversefutures" | "inverse_futures" => ContractType::InverseFutures,
            _ => ContractType::Unknown,
        }
    }

    /// Get a short abbreviation for display in compact UI elements.
    ///
    /// Returns two-letter codes: LP/LF for linear, IP/IF for inverse,
    /// with P/F suffix for perpetual/futures.
    #[allow(dead_code)]
    pub fn abbreviation(&self) -> &'static str {
        match self {
            ContractType::LinearPerpetual => "LP",
            ContractType::LinearFutures => "LF",
            ContractType::InversePerpetual => "IP",
            ContractType::InverseFutures => "IF",
            ContractType::Unknown => "??",
        }
    }

    /// Get a human-readable display name (max 8 chars).
    ///
    /// Suitable for headers, legends, and filter labels.
    pub fn display_name(&self) -> &'static str {
        match self {
            ContractType::LinearPerpetual => "LinPerp",
            ContractType::LinearFutures => "LinFut",
            ContractType::InversePerpetual => "InvPerp",
            ContractType::InverseFutures => "InvFut",
            ContractType::Unknown => "Unknown",
        }
    }

    /// Get all contract types as a slice.
    ///
    /// Useful for iteration and initialization.
    #[allow(dead_code)]
    pub fn all() -> &'static [ContractType; 4] {
        &[
            ContractType::LinearPerpetual,
            ContractType::LinearFutures,
            ContractType::InversePerpetual,
            ContractType::InverseFutures,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str_exact_matches() {
        assert_eq!(
            ContractType::from_str("LinearPerpetual"),
            ContractType::LinearPerpetual
        );
        assert_eq!(
            ContractType::from_str("LinearFutures"),
            ContractType::LinearFutures
        );
        assert_eq!(
            ContractType::from_str("InversePerpetual"),
            ContractType::InversePerpetual
        );
        assert_eq!(
            ContractType::from_str("InverseFutures"),
            ContractType::InverseFutures
        );
    }

    #[test]
    fn test_from_str_case_insensitive() {
        assert_eq!(
            ContractType::from_str("linear_perpetual"),
            ContractType::LinearPerpetual
        );
        assert_eq!(
            ContractType::from_str("INVERSE_FUTURES"),
            ContractType::InverseFutures
        );
    }

    #[test]
    fn test_from_str_unknown_returns_unknown_variant() {
        // Unknown values return ContractType::Unknown instead of silently defaulting
        assert_eq!(ContractType::from_str("Unknown"), ContractType::Unknown);
        assert_eq!(ContractType::from_str(""), ContractType::Unknown);
        assert_eq!(ContractType::from_str("spot"), ContractType::Unknown);
    }

    #[test]
    fn test_abbreviation() {
        assert_eq!(ContractType::LinearPerpetual.abbreviation(), "LP");
        assert_eq!(ContractType::LinearFutures.abbreviation(), "LF");
        assert_eq!(ContractType::InversePerpetual.abbreviation(), "IP");
        assert_eq!(ContractType::InverseFutures.abbreviation(), "IF");
        assert_eq!(ContractType::Unknown.abbreviation(), "??");
    }

    #[test]
    fn test_display_name() {
        assert_eq!(ContractType::LinearPerpetual.display_name(), "LinPerp");
        assert_eq!(ContractType::LinearFutures.display_name(), "LinFut");
        assert_eq!(ContractType::InversePerpetual.display_name(), "InvPerp");
        assert_eq!(ContractType::InverseFutures.display_name(), "InvFut");
        assert_eq!(ContractType::Unknown.display_name(), "Unknown");
    }

    #[test]
    fn test_all() {
        let all = ContractType::all();
        assert_eq!(all.len(), 4);
        assert!(all.contains(&ContractType::LinearPerpetual));
        assert!(all.contains(&ContractType::LinearFutures));
        assert!(all.contains(&ContractType::InversePerpetual));
        assert!(all.contains(&ContractType::InverseFutures));
        // Unknown is not a valid filter option
        assert!(!all.contains(&ContractType::Unknown));
    }
}
