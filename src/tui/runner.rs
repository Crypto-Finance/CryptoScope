use crate::cli::ScreenerMode;
use crate::exchange::create_exchange;
use crate::fetcher::fetch_categories;
use crate::logging;
use crate::models::ContractType;
use crate::models::symbol::Symbol;
use crate::tui::app::{AppState, AppView, Direction};
use crate::tui::screener_service::fetch_screener_data_blocking;
use anyhow::Result;
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::event::{MouseEvent, MouseEventKind};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use std::cell::RefCell;
use std::io;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tokio::time::timeout;
use tracing::info;

const SCREENER_TIMEOUT_SECS: u64 = 30;
const SCREENER_TIMEOUT_MSG: &str = "Screener fetch timed out after 30s";

type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;
type AppStateRef = Arc<RwLock<AppState>>;

/// RAII guard that suppresses logs on creation and restores them on drop.
struct LogGuard;

impl LogGuard {
    fn new() -> Self {
        logging::suppress_logs();
        Self
    }
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        logging::restore_logs();
    }
}

/// Unified lifecycle manager for the TUI application.
///
/// Handles terminal setup/teardown with proper `Drop` cleanup.
/// Log suppression is managed separately by `LogGuard`.
struct TuiLifecycle {
    terminal: AppTerminal,
    state: AppStateRef,
    _log_guard: LogGuard,
}

impl TuiLifecycle {
    /// Initialize the terminal and app state.
    ///
    /// Enables raw mode, enters alternate screen, enables mouse capture,
    /// and suppresses logs via `LogGuard`.
    fn init(
        exchange_name: &str,
        categories: &[&str],
        contract_types: &[String],
        mode: ScreenerMode,
    ) -> Result<Self> {
        let _log_guard = LogGuard::new();

        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        terminal.clear()?;

        let cat_strings: Vec<String> = categories.iter().map(ToString::to_string).collect();
        let parsed_contract_types = parse_contract_types(contract_types);
        let state = create_state_with_fetches(
            exchange_name, &cat_strings, parsed_contract_types, mode,
        );

        Ok(Self {
            terminal,
            state,
            _log_guard,
        })
    }

    fn terminal(&mut self) -> &mut AppTerminal {
        &mut self.terminal
    }

    fn state(&self) -> &AppStateRef {
        &self.state
    }
}

/// Restore terminal state. Logs are restored by `LogGuard::drop`.
///
/// `disable_raw_mode` is idempotent, so this can run unconditionally.
fn restore_terminal() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(
        io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::event::DisableMouseCapture
    )?;
    Ok(())
}

impl Drop for TuiLifecycle {
    fn drop(&mut self) {
        let _ = restore_terminal();
    }
}

/// Parse contract types from CLI strings, logging warnings for unknown types.
fn parse_contract_types(contract_types: &[String]) -> Vec<ContractType> {
    contract_types
        .iter()
        .map(|s| {
            let parsed = ContractType::from_str(s);
            if matches!(parsed, ContractType::Unknown) {
                tracing::warn!("Unknown contract type from CLI: '{}', treating as Unknown", s);
            }
            parsed
        })
        .collect()
}

/// Create app state and spawn initial data fetches.
fn create_state_with_fetches(
    exchange_name: &str,
    categories: &[String],
    parsed_contract_types: Vec<ContractType>,
    mode: ScreenerMode,
) -> AppStateRef {
    let state = Arc::new(RwLock::new(AppState::new_with_contract_types(
        exchange_name.to_string(),
        categories.to_vec(),
        parsed_contract_types,
        mode,
    )));
    let exchange = exchange_name.to_string();
    let cats = categories.to_vec();
    spawn_fetch_task(&state, exchange.clone(), cats.clone(), FetchKind::Symbols, false);
    spawn_fetch_task(
        &state,
        exchange,
        cats,
        FetchKind::Screener(mode),
        false,
    );
    state
}

/// Run the TUI application with the given exchange, categories, contract types, and screener mode.
pub async fn run(
    exchange_name: &str,
    categories: &[&str],
    contract_types: &[String],
    mode: ScreenerMode,
) -> Result<()> {
    let mut lifecycle = TuiLifecycle::init(exchange_name, categories, contract_types, mode)?;

    let state = lifecycle.state().clone();
    event_loop(lifecycle.terminal(), &state).await?;

    Ok(())
}

/// Main event loop: render, poll events, handle input.
async fn event_loop(terminal: &mut AppTerminal, state: &AppStateRef) -> Result<()> {
    loop {
        // RefCell needed: render closure is FnMut and borrows mutably;
        // can't pass &mut click_regions through closure boundary.
        let click_regions_cell = RefCell::new(crate::tui::mouse::ClickRegions::new());

        let mut state_write = state.write().await;
        terminal.draw(|frame| {
            let regions = crate::tui::widgets::render(frame, &mut state_write);
            *click_regions_cell.borrow_mut() = regions;
        })?;
        drop(state_write);

        let click_regions = click_regions_cell.into_inner();

        if poll_and_dispatch(state, &click_regions).await? {
            return Ok(());
        }

        let mut app_state = state.write().await;
        app_state.update_popup();
    }
}

/// Poll for events and dispatch to handlers. Returns `true` if the app should quit.
async fn poll_and_dispatch(
    state: &AppStateRef,
    click_regions: &crate::tui::mouse::ClickRegions,
) -> Result<bool> {
    if !event::poll(Duration::from_millis(250))? {
        return Ok(false);
    }

    let event = event::read()?;
    let mut app_state = state.write().await;

    // Handle popup dismissal first (for any event).
    // ALL events are consumed here — not just key presses — because a popup
    // represents a transient overlay that blocks interaction with the
    // underlying UI. If we forwarded the event (e.g., a mouse click on a
    // table row), the user would accidentally trigger an action they didn't
    // intend while the popup was visible. Dismissing on any event gives the
    // user an intuitive "tap anywhere to dismiss" experience.
    if app_state.popup.message.is_some() {
        app_state.dismiss_popup();
        return Ok(false);  // Dismiss popup, consume ALL events
    }

    match event {
        Event::Key(key) => {
            if key.kind != KeyEventKind::Press {
                return Ok(false);
            }
            match crate::tui::key_handler::handle_key_event(&mut app_state, key) {
                crate::tui::key_handler::NavResult::Quit => Ok(true),
                crate::tui::key_handler::NavResult::Refresh => {
                    handle_refresh(&mut app_state, state);
                    Ok(false)
                }
                crate::tui::key_handler::NavResult::Consumed
                | crate::tui::key_handler::NavResult::Ignored => Ok(false),
            }
        }
        Event::Mouse(mouse) => {
            handle_mouse_event(&mut app_state, mouse, click_regions);
            Ok(false)
        }
        _ => Ok(false),
    }
}

/// Handle refresh key: spawn appropriate fetch task for current view.
fn handle_refresh(app_state: &mut AppState, state: &AppStateRef) {
    if app_state.view == AppView::Screener {
        app_state.screener.loading = true;
        spawn_fetch_task(
            state,
            app_state.exchange_name.clone(),
            app_state.categories.clone(),
            FetchKind::Screener(app_state.screener_mode),
            true,
        );
    } else {
        app_state.loading = true;
        spawn_fetch_task(
            state,
            app_state.exchange_name.clone(),
            app_state.categories.clone(),
            FetchKind::Symbols,
            true,
        );
    }
    app_state.dismiss_popup();
}

/// Handle a mouse event by hit-testing against click regions.
fn handle_mouse_event(
    app_state: &mut AppState,
    mouse: MouseEvent,
    click_regions: &crate::tui::mouse::ClickRegions,
) {
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            app_state.scroll_view(Direction::Next);
        }
        MouseEventKind::ScrollUp => {
            app_state.scroll_view(Direction::Previous);
        }
        MouseEventKind::Down(_) => {
            if let Some(action) = click_regions.hit_test(mouse.column, mouse.row) {
                app_state.on_mouse_click(action);
            }
        }
        _ => {}
    }
}

/// Which kind of fetch to perform (symbol list vs screener).
enum FetchKind {
    Symbols,
    Screener(ScreenerMode),
}

/// Unified spawn function for all fetch/refresh tasks.
fn spawn_fetch_task(
    state: &AppStateRef,
    exchange: String,
    categories: Vec<String>,
    kind: FetchKind,
    is_refresh: bool,
) {
    let state_clone = state.clone();

    tokio::spawn(async move {
        let action = if is_refresh { "refresh" } else { "fetch" };
        let label = &exchange;
        info!("Starting async {action} for exchange: {label}");

        match kind {
            FetchKind::Symbols => {
                do_fetch(state_clone, &exchange, &categories, is_refresh).await;
            }
            FetchKind::Screener(mode) => {
                do_screener_fetch(state_clone, exchange, categories, mode, is_refresh).await;
            }
        }
    });
}

/// Handle successful symbol fetch: update state and optionally show popup.
async fn handle_fetch_success(state: &AppStateRef, symbols: Vec<Symbol>, is_refresh: bool) {
    info!("Fetched {} symbols", symbols.len());
    let mut s = state.write().await;
    s.set_symbols(symbols);
    if is_refresh {
        s.show_popup("Refresh complete".to_string(), false);
    }
}

/// Handle fetch error: show error popup with appropriate prefix.
async fn handle_fetch_error(state: &AppStateRef, error: anyhow::Error, is_refresh: bool) {
    let mut s = state.write().await;
    let prefix = if is_refresh { "Refresh" } else { "Fetch" };
    s.show_popup(format!("{prefix} failed: {error}"), true);
}

/// Show an error popup after acquiring write lock.
async fn show_error_popup(state: &AppStateRef, message: String) {
    state.write().await.show_popup(message, true);
}

/// Show an error popup only if generation still matches.
async fn show_error_popup_if_current(
    state: &AppStateRef,
    generation: u64,
    message: String,
) {
    let mut s = state.write().await;
    if s.screener.generation == generation {
        s.screener.loading = false;
        s.show_popup(message, true);
    }
}

async fn do_fetch(
    state: AppStateRef,
    exchange: &str,
    categories: &[String],
    is_refresh: bool,
) {
    let cat_refs: Vec<&str> = categories.iter().map(|s| s.as_str()).collect();

    match create_exchange(exchange) {
        Ok(exchange_client) => {
            match fetch_categories(&*exchange_client, &cat_refs).await {
                Ok(symbols) => handle_fetch_success(&state, symbols, is_refresh).await,
                Err(e) => handle_fetch_error(&state, e.into(), is_refresh).await,
            }
        }
        Err(e) => {
            show_error_popup(&state, format!("Exchange error: {e}")).await;
        }
    }
}

/// Apply screener fetch results to app state (handles generation mismatch, errors, popups).
async fn apply_screener_results(
    state: &AppStateRef,
    result: Result<Vec<crate::models::price::PriceChange>, anyhow::Error>,
    generation: u64,
    is_refresh: bool,
) {
    match result {
        Ok(results) => {
            info!("Screener fetched {} results", results.len());
            let mut s = state.write().await;
            if !s.set_screener_results(results, generation) {
                info!("Discarding stale screener results (generation mismatch)");
                // Don't touch loading flag — newer fetch owns it
            } else if is_refresh {
                s.show_popup("Screener refresh complete".to_string(), false);
            }
        }
        Err(e) => {
            let prefix = if is_refresh { "Screener refresh" } else { "Screener" };
            show_error_popup_if_current(state, generation, format!("{prefix} failed: {e}")).await;
        }
    }
}

async fn handle_screener_cancel(state: &AppStateRef, generation: u64) {
    show_error_popup_if_current(state, generation, "Screener fetch cancelled".to_string()).await;
}

async fn handle_screener_timeout(state: &AppStateRef, generation: u64) {
    tracing::warn!("{SCREENER_TIMEOUT_MSG}");
    show_error_popup_if_current(state, generation, SCREENER_TIMEOUT_MSG.to_string()).await;
}

async fn do_screener_fetch(
    state: AppStateRef,
    exchange: String,
    categories: Vec<String>,
    mode: ScreenerMode,
    is_refresh: bool,
) {
    // Increment generation before fetch to tag this request
    let generation = {
        let mut s = state.write().await;
        s.screener.generation += 1;
        s.screener.loading = true;
        s.screener.generation
    };

    // Use oneshot channel to bridge sync→async
    let (tx, rx) = tokio::sync::oneshot::channel();
    let exchange_clone = exchange.clone();
    let cats: Vec<String> = categories.clone();

    tokio::task::spawn_blocking(move || {
        let cat_refs: Vec<&str> = cats.iter().map(|s| s.as_str()).collect();
        let result = fetch_screener_data_blocking(&exchange_clone, &cat_refs, mode);
        let _ = tx.send(result);
    });

    // Receive result in async context with 30s timeout
    let result = match timeout(Duration::from_secs(SCREENER_TIMEOUT_SECS), rx).await {
        Ok(Ok(result)) => result,
        Ok(Err(_)) => {
            handle_screener_cancel(&state, generation).await;
            return;
        }
        Err(_) => {
            handle_screener_timeout(&state, generation).await;
            return;
        }
    };

    apply_screener_results(&state, result, generation, is_refresh).await;
}
