use super::style;
use crate::app::{Game, Phase};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

pub fn info(g: &Game) -> Vec<Line<'static>> {
    let dice: Vec<String> = g.dice.iter().map(|&d| char::from_u32(0x267f + d as u32).unwrap().to_string()).collect();
    let owner = g.cube.owner.map_or("center".into(), |p| format!("{p:?}"));
    let mode = g.ai.map_or("hotseat".into(), |p| format!("{:?} AI as {p:?}", g.level));
    let turn = Span::styled(format!("● {:?}", g.turn), Style::default().fg(style::player(g.turn)));
    vec![
        Line::from(vec![Span::raw("Turn   "), turn]),
        Line::raw(format!("Dice   {}", if dice.is_empty() { "-".into() } else { dice.join(" ") })),
        Line::raw(format!("Cube   {} ({owner})", g.cube.value)),
        Line::raw(format!("Score  White {}  Black {}", g.score[0], g.score[1])),
        Line::raw(format!("Mode   {mode}")),
        Line::raw(""),
        Line::styled(prompt(g), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
    ]
}

pub fn keys() -> Vec<Line<'static>> {
    ["←/→ move   enter pick/drop   esc cancel", "r roll   d double   y take   p pass", "a level   m mode   n new   q quit"]
        .map(Line::raw)
        .to_vec()
}

fn prompt(g: &Game) -> String {
    match g.phase {
        Phase::Roll if g.cube.can_double(g.turn) => "Press r to roll or d to double".into(),
        Phase::Roll => "Press r to roll".into(),
        Phase::Move => g.msg.clone(),
        Phase::Offer => format!("{:?}: y take / p pass", g.actor()),
        Phase::Over => "Game over, press n".into(),
    }
}
