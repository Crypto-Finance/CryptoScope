use crate::tui::app::AppState;
use crate::tui::theme::CyberdeckTheme;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;

pub fn render(frame: &mut Frame, area: Rect, state: &AppState) {
    let mut spans = vec![];

    if state.loading {
        spans.push(Span::styled(
            " ⏳ Refreshing... ",
            Style::default()
                .fg(CyberdeckTheme::WHITE)
                .bg(CyberdeckTheme::DARK_BG)
                .add_modifier(Modifier::BOLD),
        ));
    }

    if state.search_mode {
        spans.push(Span::styled(
            format!(" 🔍 /{} ", state.search),
            CyberdeckTheme::footer_style(),
        ));
    }

    let count_text = format!(" Rows: {} ", state.filtered.len());
    spans.push(Span::styled(
        &count_text,
        Style::default()
            .fg(CyberdeckTheme::WHITE)
            .bg(CyberdeckTheme::DARK_BG)
            .add_modifier(Modifier::BOLD),
    ));

    let view_text = match &state.view {
        crate::tui::app::AppView::SymbolList => " [Tab]Stats ",
        crate::tui::app::AppView::StatsDashboard => " [Tab]List ",
    };
    spans.push(Span::styled(
        view_text,
        Style::default()
            .fg(CyberdeckTheme::WHITE)
            .bg(CyberdeckTheme::DARK_BG)
            .add_modifier(Modifier::BOLD),
    ));

    let keys = " [q/Esc]quit [j/k]nav [/]search [r]efresh ";
    spans.push(Span::styled(
        keys,
        Style::default()
            .fg(CyberdeckTheme::DIM)
            .bg(CyberdeckTheme::BLACK),
    ));

    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(Style::default().bg(CyberdeckTheme::BLACK));
    frame.render_widget(paragraph, area);
}
