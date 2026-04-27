use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{Block, BorderType, Borders};

// Cyberpunk-inspired color palette for the TUI application.
pub const BLACK: Color = Color::Rgb(0, 0, 0);
pub const DARK_BG: Color = Color::Rgb(34, 37, 41);
pub const LINE: Color = Color::Rgb(57, 62, 70);
pub const TAG: Color = Color::Rgb(86, 130, 177);
pub const WHITE: Color = Color::Rgb(200, 200, 200);
pub const DIM: Color = Color::Rgb(100, 100, 120);
pub const RED: Color = Color::Rgb(255, 80, 80);
pub const BLUE: Color = Color::Rgb(47, 47, 228);

// Price change colors for screener
pub const PRICE_UP: Color = Color::Rgb(0, 200, 100);
pub const PRICE_DOWN: Color = Color::Rgb(255, 80, 80);

/// Get color for price change percentage
pub fn change_color(percent: f64) -> Color {
    if percent >= 0.0 {
        PRICE_UP
    } else {
        PRICE_DOWN
    }
}

/// Create a styled block with the cyberdeck theme.
///
/// Returns a rounded-bordered block with themed colors,
/// suitable for panels, tables, and stat displays.
pub fn themed_block(title: &str) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(LINE))
        .title(format!(" {title} "))
        .title_style(Style::default().fg(BLUE).add_modifier(Modifier::BOLD))
        .style(Style::default().fg(WHITE).bg(BLACK))
}

pub fn footer_style() -> Style {
    Style::default()
        .fg(BLACK)
        .bg(LINE)
        .add_modifier(Modifier::BOLD)
}

/// Style for footer item spans (white text on dark background, bold).
pub fn footer_item_style() -> Style {
    Style::default()
        .fg(WHITE)
        .bg(DARK_BG)
        .add_modifier(Modifier::BOLD)
}
