//! Screener mode configuration.

#![allow(dead_code)]

/// Screener mode: Ticker (fast, rolling 24h) or Kline (accurate, true daily open).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ScreenerMode {
    /// Ticker mode uses rolling 24h price (prev_price_24h) - inaccurate for daily open
    Ticker,
    /// Kline mode uses true 00:00 UTC daily open from K-line endpoint - accurate
    #[default]
    Kline,
}
