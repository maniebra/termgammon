use super::style;
use crate::app::{Game, Phase};
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn info(g: &Game) -> Vec<Line<'static>> {
    let owner = g.cube.owner.map_or("center".into(), |p| format!("{p:?}"));
    let mode = g.ai.map_or("hotseat".into(), |p| format!("{:?} AI as {p:?}", g.level));
    let turn = Span::styled(format!("● {:?}", g.turn), Style::default().fg(style::player(g.turn)));
    let mut lines = vec![Line::from(vec![Span::raw("Turn   "), turn])];
    lines.extend(super::dice::art(g));
    lines.extend([
        Line::raw(format!("Cube   {} ({owner}){}", g.cube.value, if g.crawford { "  crawford" } else { "" })),
        Line::raw(format!("Score  White {}  Black {}  (to {})", g.score[0], g.score[1], g.target)),
        Line::raw(format!("Mode   {mode}")),
        Line::raw(""),
        Line::styled(prompt(g), Style::default().fg(style::accent()).add_modifier(Modifier::BOLD)),
    ]);
    lines
}

pub fn keys() -> Vec<Line<'static>> {
    ["arrows move   tab next   enter pick/drop", "u undo   ? hint   r roll   d double   y/p", "esc / q menu"]
        .map(Line::raw)
        .to_vec()
}

pub fn prompt(g: &Game) -> String {
    match g.phase {
        Phase::Roll if g.may_double() => "Press r to roll or d to double".into(),
        Phase::Roll => "Press r to roll".into(),
        _ if g.thinking() => format!("{:?} is thinking {}", g.turn, SPIN[g.frame as usize / 3 % SPIN.len()]),
        Phase::Move if !g.msg.is_empty() => g.msg.clone(),
        Phase::Move if g.source().is_some() => "Move to a green spot (enter / click)".into(),
        Phase::Move => "Pick a checker to move".into(),
        Phase::Offer => format!("{:?}: y take / p pass", g.actor()),
        Phase::Over => "Enter: next game   Esc: menu".into(),
    }
}

const SPIN: [&str; 4] = ["◐", "◓", "◑", "◒"];
