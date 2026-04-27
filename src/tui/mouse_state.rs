use crate::tui::app::{AppView, Direction};
use crate::tui::mouse::{ClickAction, ScrollDirection};
use crate::tui::widgets::table_common::scroll_select;
use ratatui::widgets::TableState;

/// Bundles a table state with its filtered item count for mouse operations.
pub struct TableContext<'a> {
    pub state: &'a mut TableState,
    pub filtered_len: usize,
}

/// Handles mouse click dispatch for the TUI.
///
/// This is a stateless handler — it takes mutable references to the
/// table states it needs to modify.
pub struct MouseState;

impl MouseState {
    /// Handle a table row click (used for both symbol list and screener tables).
    pub fn on_table_click(table_state: &mut TableState, row_index: usize, filtered_len: usize) {
        if row_index < filtered_len {
            table_state.select(Some(row_index));
        }
    }

    /// Handle scrollbar track click (page up/down).
    pub fn on_scrollbar_track_click(
        table_state: &mut TableState,
        filtered_len: usize,
        direction: ScrollDirection,
    ) {
        if filtered_len == 0 {
            return;
        }
        let jump = 10.max(filtered_len / 5);
        match direction {
            ScrollDirection::Up => {
                let current = table_state.selected().unwrap_or(0);
                let new_pos = current.saturating_sub(jump);
                table_state.select(Some(new_pos));
            }
            ScrollDirection::Down => {
                let current = table_state.selected().unwrap_or(0);
                let new_pos = (current + jump).min(filtered_len.saturating_sub(1));
                table_state.select(Some(new_pos));
            }
        }
    }

    /// Scroll in the appropriate view (Screener uses its own table state).
    pub fn scroll_view(
        view: AppView,
        screener: TableContext<'_>,
        symbol: TableContext<'_>,
        direction: Direction,
    ) {
        let ctx = if matches!(view, AppView::Screener) { screener } else { symbol };
        scroll_select(ctx.state, ctx.filtered_len, direction);
    }

    /// Handle a mouse click action by dispatching to the appropriate handler.
    pub fn on_click(
        action: &ClickAction,
        view: AppView,
        screener: TableContext<'_>,
        symbol: TableContext<'_>,
    ) {
        match action {
            ClickAction::ScrollUp => {
                Self::scroll_view(view, screener, symbol, Direction::Previous);
            }
            ClickAction::ScrollDown => {
                Self::scroll_view(view, screener, symbol, Direction::Next);
            }
            ClickAction::TableRow(index) => {
                Self::on_table_click(symbol.state, *index, symbol.filtered_len);
            }
            ClickAction::ScreenerTableRow(index) => {
                Self::on_table_click(screener.state, *index, screener.filtered_len);
            }
            ClickAction::ScrollbarTrack(direction) => {
                // Scrollbar track clicks apply to the current view's table
                if matches!(view, AppView::Screener) {
                    Self::on_scrollbar_track_click(
                        screener.state,
                        screener.filtered_len,
                        *direction,
                    );
                } else {
                    Self::on_scrollbar_track_click(
                        symbol.state,
                        symbol.filtered_len,
                        *direction,
                    );
                }
            }
        }
    }
}
