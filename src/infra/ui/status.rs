use crate::app::{Game, Phase};
use ratatui::text::Line;

pub fn lines(g: &Game) -> Vec<Line<'static>> {
    let owner = g.cube.owner.map_or("center".into(), |p| format!("{p:?}"));
    let mode = g.ai.map_or("hotseat".into(), |p| format!("{:?} AI plays {p:?}", g.level));
    vec![
        Line::raw(format!(" Turn: {:?}   Dice: {:?}   Cube: {} ({owner})", g.turn, g.dice, g.cube.value)),
        Line::raw(format!(" Score  W:{}  B:{}   Mode: {mode}", g.score[0], g.score[1])),
        Line::raw(format!(" {}", prompt(g))),
        Line::raw(""),
        Line::raw(" ←/→ move  Enter select/drop  Esc cancel  a level  m mode  n new  q quit"),
        Line::raw(""),
    ]
}

fn prompt(g: &Game) -> String {
    match g.phase {
        Phase::Roll if g.cube.can_double(g.turn) => "r roll  d double".into(),
        Phase::Roll => "r roll".into(),
        Phase::Move => g.msg.clone(),
        Phase::Offer => format!("{:?}: y take  p pass", g.actor()),
        Phase::Over => "n new game".into(),
    }
}
