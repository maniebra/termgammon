use crate::infra::ui::style;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Clear, Paragraph},
};

const LIFE: u32 = 40;

pub fn draw(f: &mut Frame, board: Rect, text: &str, age: u32, frame: u32) {
    if age >= LIFE {
        return;
    }
    let shown = ((age as usize * 3) / 2 + 1).min(text.chars().count());
    let spans: Vec<Span> = text
        .chars()
        .take(shown)
        .enumerate()
        .map(|(i, c)| {
            let color = if age > LIFE - 8 { Color::DarkGray } else { style::glow(frame * 2 + i as u32 * 6) };
            Span::styled(format!("{c} "), Style::default().fg(color).add_modifier(Modifier::BOLD))
        })
        .collect();
    let width = text.chars().count() as u16 * 2 + 3;
    let [area] = Layout::vertical([Constraint::Length(3)]).flex(Flex::Center).areas(board);
    let [area] = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center).areas(area);
    let block =
        Block::bordered().border_type(BorderType::Double).border_style(Style::default().fg(style::glow(frame * 3)));
    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(Line::from(spans)).centered().block(block), area);
}
