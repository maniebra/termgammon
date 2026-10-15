use super::style;
use crate::app::Game;
use crate::domain::Player;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub const ROWS: usize = 5;

pub fn checker(g: &Game, i: usize, row: usize, targets: &[Option<usize>]) -> Span<'static> {
    let v = g.board.points[i];
    let n = v.unsigned_abs() as usize;
    let owner = if v > 0 { Player::White } else { Player::Black };
    let text = match row {
        r if r >= n => "   ".to_string(),
        r if r == ROWS - 1 && n > ROWS => format!("{n:>2} "),
        _ => " ● ".to_string(),
    };
    let base = Style::default().bg(style::point_bg(i)).fg(style::player(owner));
    Span::styled(text, style::mark(g, i, targets, base))
}

pub fn label(g: &Game, i: usize, targets: &[Option<usize>]) -> Span<'static> {
    let base = Style::default().fg(Color::DarkGray);
    Span::styled(format!("{:^3}", i + 1), style::mark(g, i, targets, base))
}
