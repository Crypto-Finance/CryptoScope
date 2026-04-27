mod cli;
mod db;
mod error;
mod exchange;
mod fetcher;
mod logging;
mod models;
mod output;
mod screener;
mod tui;
mod utils;

#[cfg(test)]
mod test_utils;

use anyhow::Result;
use std::sync::Arc;
use std::time::Instant;
use tracing::info;

use cli::{Cli, Commands, OutputMode, ScreenerMode, parse_categories};
use db::{Database, create_connection, init_schema};
use exchange::create_exchange;
use fetcher::fetch_categories;
use models::{PriceChange, Statistics, Symbol};
use output::{apply_filter, format_text, print_json};
use screener::{Screener, display_output, display_output_stats};

/// Print screener results and stats to stdout.
fn print_screener_output(
    changes: &[PriceChange],
    top: Option<usize>,
    min_change: Option<f64>,
    min_volume: Option<f64>,
    symbol: Option<&str>,
) {
    display_output(changes, top, min_change, min_volume, symbol);
    display_output_stats(changes);
}

async fn run_screener(cmd: &cli::ScreenerCmd) -> Result<()> {
    let conn = create_connection()?;
    init_schema(&conn)?;
    let mut db = Database::new(conn);

    let exchange = create_exchange(&cmd.common.exchange)?;
    info!("Created exchange client: {}", exchange.name());

    let categories: Vec<String> = parse_categories(&cmd.common.category)?
        .into_iter()
        .map(|s| s.to_string())
        .collect();
    info!("Screener categories: {:?}", categories);

    if cmd.force_refresh {
        info!("Force refresh enabled, clearing cache");
        db.clear_price_data()?;
    }

    let mut screener = Screener::new(db, Arc::from(exchange), cmd.mode, categories);
    let changes = screener.run().await?;

    print_screener_output(
        &changes,
        cmd.top,
        cmd.min_change,
        cmd.min_volume,
        cmd.symbol.as_deref(),
    );

    // Hint for cache refresh
    println!();
    eprintln!("[i] Tip: Use --force-refresh to clear stale cache data");

    Ok(())
}

/// Fetch symbols, apply filters, and return the result along with parsed categories.
async fn fetch_and_filter(cli: &Cli) -> Result<(Vec<Symbol>, Statistics, Vec<&'static str>)> {
    let exchange = create_exchange(&cli.common.exchange)?;
    info!("Created exchange client: {}", exchange.name());

    let categories = parse_categories(&cli.common.category)?;
    info!("Fetching categories: {:?}", categories);

    let all_symbols = fetch_categories(&*exchange, &categories).await?;
    info!("Total symbols fetched: {}", all_symbols.len());

    let filtered_symbols = apply_filter(&all_symbols, cli.search.as_deref());

    if cli.search.is_some() {
        info!(
            "Filtered from {} to {} symbols",
            all_symbols.len(),
            filtered_symbols.len()
        );
    }

    let stats = Statistics::from_symbols(&filtered_symbols);
    Ok((filtered_symbols, stats, categories))
}

/// Compute and display results based on output mode.
async fn compute_and_display(
    cli: &Cli,
    symbols: &[Symbol],
    stats: &Statistics,
    categories: &[&'static str],
    elapsed: std::time::Duration,
) -> Result<()> {
    // Extract screener mode from CLI if available, default to Kline
    let screener_mode = match &cli.command {
        Some(Commands::Screener(cmd)) => cmd.mode,
        None => ScreenerMode::default(),
    };

    match cli.get_output_mode() {
        OutputMode::Json => {
            print_json(&cli.common.exchange, categories, symbols, stats)?;
        }
        OutputMode::Tui => {
            tui::run(
                &cli.common.exchange,
                categories,
                &cli.common.contract_type,
                screener_mode,
            )
            .await?;
        }
        OutputMode::Text => {
            format_text(&cli.common.exchange, categories, symbols, stats);
            println!("[OK] Fetch completed in {:.1}s", elapsed.as_secs_f64());
        }
    }
    Ok(())
}

/// Dispatch to the appropriate command handler.
async fn dispatch_command(cli: &Cli) -> Result<()> {
    if let Some(Commands::Screener(cmd)) = &cli.command {
        info!("Running screener subcommand");
        return run_screener(cmd).await;
    }

    info!(
        "Exchange: {}, Category: {:?}, Output: {:?}",
        cli.common.exchange,
        parse_categories(&cli.common.category)?,
        cli.get_output_mode()
    );

    let start_time = Instant::now();
    let (symbols, stats, categories) = fetch_and_filter(cli).await?;
    let elapsed = start_time.elapsed();
    compute_and_display(cli, &symbols, &stats, &categories, elapsed).await
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse_args();
    logging::init_logging(cli.verbose);

    info!("Starting CryptoScope...");

    dispatch_command(&cli).await
}
