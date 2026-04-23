pub mod footer;
pub mod header;
pub mod popup;
pub mod stats_panel;
pub mod symbol_table;

use crate::tui::app::AppState;
use ratatui::Frame;

pub fn render(frame: &mut Frame, state: &AppState) {
    use ratatui::layout::{Constraint, Direction, Layout};

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(frame.area());

    header::render(frame, chunks[0], state);

    match &state.view {
        crate::tui::app::AppView::SymbolList => {
            symbol_table::render(frame, chunks[1], state);
        }
        crate::tui::app::AppView::StatsDashboard => {
            stats_panel::render(frame, chunks[1], state);
        }
    }

    footer::render(frame, chunks[2], state);

    if let Some(ref msg) = state.popup_message {
        popup::render_popup(frame, msg, state.popup_is_error);
    }
}
