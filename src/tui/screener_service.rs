//! Sync screener service for TUI integration.
//!
//! Provides a blocking entry point for screener data fetching,
//! designed to be called from `tokio::task::spawn_blocking`.

use crate::cli::ScreenerMode;
use crate::db::{Database, create_connection, init_schema};
use crate::exchange::create_exchange;
use crate::models::price::PriceChange;
use crate::screener::Screener;
use anyhow::Result;
use std::sync::Arc;

/// Fetch screener data from a blocking context.
///
/// Called from `tokio::task::spawn_blocking` to keep the TUI event loop
/// responsive. Since `rusqlite::Connection` is `!Send`, we cannot use
/// `tokio::spawn` directly; instead we build a scoped `new_current_thread`
/// runtime inside the blocking thread to execute the async screener.
pub fn fetch_screener_data_blocking(
    exchange_name: &str,
    categories: &[&str],
    mode: ScreenerMode,
) -> Result<Vec<PriceChange>> {
    let exchange_name = exchange_name.to_string();
    let categories: Vec<String> = categories.iter().map(|s| s.to_string()).collect();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to build runtime: {}", e))?;

    rt.block_on(async {
        let conn = create_connection()?;
        init_schema(&conn)?;
        let db = Database::new(conn);
        let exchange = create_exchange(&exchange_name)?;
        let mut screener = Screener::new(db, Arc::from(exchange), mode, categories);
        screener.run().await
    })
    .map_err(|e| anyhow::anyhow!(e))
}
