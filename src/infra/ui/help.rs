use super::style;
use crate::app::Game;
use crate::domain::Player;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn banner(frame: u32) -> Vec<Line<'static>> {
    let title = "T E R M G A M M O N".chars().enumerate().map(|(i, c)| {
        let wave = Style::default().fg(style::glow(frame * 4 + i as u32 * 12)).add_modifier(Modifier::BOLD);
        Span::styled(c.to_string(), wave)
    });
    let row = |shift: u32| {
        let spans = (0..8u32).map(move |i| {
            let color =
                style::player(if (i + shift + frame / 8).is_multiple_of(2) { Player::White } else { Player::Black });
            Span::styled("● ", Style::default().fg(color))
        });
        Line::from(spans.collect::<Vec<_>>())
    };
    vec![row(0), Line::from(title.collect::<Vec<_>>()), row(1)]
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
        "arrows move cursor (↑/↓ switch rows)   tab next playable",
        "enter or click: pick a checker, then a green spot",
        "u undo   ? hint   esc menu",
        "",
        "Gammon (loser bore off none) scores 2×, backgammon 3×.",
    ]
    .map(Line::raw)
    .to_vec()
}
