use crate::tui::app::AppState;
use crate::tui::mouse::{ClickAction, ClickRegions, ScrollDirection};
use crate::tui::theme::CyberdeckTheme;
use ratatui::Frame;
use ratatui::layout::{Layout, Rect, Direction, Constraint};
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Table, Scrollbar, ScrollbarOrientation, ScrollbarState};

/// Build the table header and data rows from the current app state.
fn build_table_rows(state: &AppState) -> (Row<'_>, Vec<Row<'_>>) {
    let header = Row::new(vec![
        Cell::from("Symbol"),
        Cell::from("Contract"),
        Cell::from("Base/Quote"),
    ])
    .style(
        Style::default()
            .fg(CyberdeckTheme::DARK_BG)
            .add_modifier(Modifier::BOLD),
    );

    let selected = state.table_state.selected();

    let rows: Vec<Row> = state
        .filtered
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let base_quote = format!("{}/{}", s.base_coin(), s.quote_coin());

            let cells = vec![
                Cell::from(s.symbol.clone()),
                Cell::from(s.contract_type()),
                Cell::from(base_quote),
            ];

            let mut row = Row::new(cells).style(Style::default().bg(CyberdeckTheme::BLACK));

            if let Some(sel) = selected
                && i == sel
            {
                row = row.style(
                    Style::default()
                        .fg(CyberdeckTheme::HIGHLIGHT_BG)
                        .bg(CyberdeckTheme::BLACK)
                        .add_modifier(Modifier::BOLD),
                );
            }

            row
        })
        .collect();

    (header, rows)
}

/// Add click regions for visible table rows and scrollbar track.
fn add_click_regions(click_regions: &mut ClickRegions, content_area: Rect, state: &AppState) {
    let row_height = 1u16;
    let header_height = 1u16;
    let title_height = 1u16;
    let top_border = 1u16;
    let content_start_y = content_area.y + title_height + top_border;

    let offset = state.table_state.offset();
    let visible_rows = content_area.height.saturating_sub(header_height) as usize;

    for (visible_idx, symbol_idx) in (offset..state.filtered.len()).take(visible_rows).enumerate() {
        let row_y = content_start_y + header_height + (visible_idx as u16 * row_height);
        if row_y < content_area.y + content_area.height {
            click_regions.add(
                Rect::new(content_area.x, row_y, content_area.width, row_height),
                ClickAction::TableRow(symbol_idx),
            );
        }
    }

    // Scrollbar track click regions (upper half = page up, lower half = page down)
    let scrollbar_width = 1u16;
    let scrollbar_x = content_area.x + content_area.width;
    let scrollbar_area = Rect::new(scrollbar_x, content_area.y, scrollbar_width, content_area.height);

    if !scrollbar_area.is_empty() && !state.filtered.is_empty() && scrollbar_area.height > 2 {
        let track_start_y = scrollbar_area.y + 1;
        let track_height = scrollbar_area.height - 2;
        let upper_half = track_height / 2;

        if upper_half > 0 {
            click_regions.add(
                Rect::new(scrollbar_area.x, track_start_y, 1, upper_half),
                ClickAction::ScrollbarTrack(ScrollDirection::Up),
            );

            let lower_start_y = track_start_y + upper_half;
            let lower_height = track_height - upper_half;
            if lower_height > 0 {
                click_regions.add(
                    Rect::new(scrollbar_area.x, lower_start_y, 1, lower_height),
                    ClickAction::ScrollbarTrack(ScrollDirection::Down),
                );
            }
        }
    }
}

/// Render the scrollbar on the right side of the table.
fn render_scrollbar(frame: &mut Frame, scrollbar_area: Rect, state: &AppState) {
    if scrollbar_area.is_empty() {
        return;
    }

    let header_height = 1u16;
    let title_height = 1u16;
    let top_border = 1u16;
    let chrome = header_height + title_height + top_border;
    let content_height = scrollbar_area.height.saturating_sub(chrome) as usize;

    let total_rows = state.filtered.len();
    let offset = state.table_state.offset();
    let max_position = total_rows.saturating_sub(content_height);
    let mut scrollbar_state = ScrollbarState::new(max_position)
        .position(offset.min(max_position));

    let scrollbar = Scrollbar::default()
        .orientation(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("▲"))
        .end_symbol(Some("▼"))
        .thumb_symbol("│")
        .track_symbol(Some("║"))
        .style(Style::default().fg(CyberdeckTheme::TAG));

    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
}

/// Render the symbol table widget with scrollbar.
///
/// Displays a scrollable table with symbol, contract type, and base/quote
/// columns. The currently selected row is highlighted.
/// A vertical scrollbar is rendered on the right side (1 char width).
pub fn render(
    frame: &mut Frame,
    area: Rect,
    state: &mut AppState,
    click_regions: &mut ClickRegions,
) {
    // Split area: content (width-1) + scrollbar (width=1)
    let [content_area, scrollbar_area] = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .areas(area);

    let widths = [20, 22, 18];
    let (header, rows) = build_table_rows(state);

    let table = Table::new(rows, widths)
        .header(header)
        .block(CyberdeckTheme::themed_block("  List Crypto "))
        .row_highlight_style(
            Style::default()
                .fg(CyberdeckTheme::TAG)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    add_click_regions(click_regions, content_area, state);

    // SAFETY: `render_stateful_widget` requires `&mut TableState`.
    // We have exclusive access via the `RwLock<AppState>` write guard in runner.rs.
    #[allow(clippy::mutable_key_type)]
    let mut ts = state.table_state.clone();
    frame.render_stateful_widget(table, content_area, &mut ts);

    // SYNC: Copy offset back to actual state after rendering
    // This preserves ratatui's automatic offset management for navigation
    *state.table_state.offset_mut() = ts.offset();

    render_scrollbar(frame, scrollbar_area, state);
}
