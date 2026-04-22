mod cli;
mod error;
mod exchange;
mod fetcher;
mod models;
mod output;

use anyhow::Result;
use std::time::Instant;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

use cli::Cli;
use exchange::create_exchange;
use fetcher::InstrumentFetcher;
use models::Statistics;
use output::{SymbolFilter, TextFormatter, print_json};

/// Run the main application logic
///
/// Handles fetching, filtering, statistics calculation, and output.
async fn run(cli: Cli) -> Result<()> {
    // Start timing
    let start_time = Instant::now();

    // Create exchange client
    let exchange = create_exchange(&cli.exchange)?;
    info!("Created exchange client: {}", exchange.name());

    // Parse categories
    let categories = cli.get_categories();
    info!("Fetching categories: {:?}", categories);

    // Fetch instruments
    let all_symbols = InstrumentFetcher::fetch_categories(&*exchange, &categories).await?;
    info!("Total symbols fetched: {}", all_symbols.len());

    // Apply filters if specified
    let filtered_symbols =
        SymbolFilter::apply(&all_symbols, cli.search.as_deref(), cli.status.as_deref());

    if cli.search.is_some() || cli.status.is_some() {
        info!(
            "Filtered from {} to {} symbols",
            all_symbols.len(),
            filtered_symbols.len()
        );
    }

    // Calculate statistics
    let stats = Statistics::from_symbols(&filtered_symbols);

    // Calculate elapsed time
    let elapsed = start_time.elapsed();

    // Output results
    if cli.is_json_output() {
        print_json(&cli.exchange, &categories, &filtered_symbols, &stats)?;
    } else {
        TextFormatter::format(&cli.exchange, &categories, &filtered_symbols, &stats);
        println!("✅ Fetch completed in {:.1}s", elapsed.as_secs_f64());
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse_args();

    // Initialize logging
    let log_level = if cli.verbose {
        Level::DEBUG
    } else {
        Level::INFO
    };
    FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();

    info!("Starting CryptoScope...");
    info!(
        "Exchange: {}, Category: {}, Output: {}",
        cli.exchange, cli.category, cli.output
    );

    run(cli).await
}
