use super::style;
use crate::app::Game;
use ratatui::{
    style::{Color, Style},
    text::Line,
};

pub fn banner() -> Vec<Line<'static>> {
    let style = Style::default().fg(Color::LightRed);
    ["●  ○  ●  ○  ●  ○", "  t e r m g a m m o n", "○  ●  ○  ●  ○  ●"].map(|s| Line::styled(s, style)).to_vec()
}

pub fn result(g: &Game) -> Vec<Line<'static>> {
    let w = g.match_winner().unwrap_or(g.turn);
    let style = Style::default().fg(style::player(w));
    vec![
        Line::styled(format!("★ {w:?} wins the match"), style),
        Line::raw(format!("White {}  ·  Black {}", g.score[0], g.score[1])),
    ]
}

pub fn rules() -> Vec<Line<'static>> {
    [
        "Move all 15 checkers home, then bear them off.",
        "White moves 24 → 1, Black moves 1 → 24.",
        "A single checker (blot) can be hit and sent to the bar.",
        "Checkers on the bar must re-enter before anything else.",
        "",
        "r roll   d double   y take   p pass",
        "←/→ or tab move cursor   enter or click pick / drop",
        "u undo   esc menu",
        "",
        "Gammon (loser bore off none) scores 2×, backgammon 3×.",
    ]
    .map(Line::raw)
    .to_vec()
}
