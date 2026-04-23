use crate::tui::app::AppState;
use crate::tui::theme::CyberdeckTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::widgets::{Cell, Row, Table};

/// Render the symbol table widget.
///
/// Displays a scrollable table with symbol, contract type, and base/quote
/// columns. The currently selected row is highlighted.
pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let widths = [20, 22, 18];

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

    let table = Table::new(rows, widths)
        .header(header)
        .block(CyberdeckTheme::themed_block(" List Crypto "))
        .row_highlight_style(
            Style::default()
                .fg(CyberdeckTheme::BLUE)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▸ ");

    // SAFETY: `render_stateful_widget` requires `&mut TableState`.
    // We have exclusive access via the `RwLock<AppState>` write guard in runner.rs.
    #[allow(clippy::mutable_key_type)]
    let mut ts = state.table_state.clone();
    frame.render_stateful_widget(table, area, &mut ts);
}
