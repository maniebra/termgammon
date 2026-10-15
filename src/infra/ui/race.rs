use super::style;
use crate::app::Game;
use crate::domain::Player;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::Style,
    widgets::{Block, LineGauge},
};

const START_PIPS: f64 = 167.0;

pub fn draw(f: &mut Frame, area: Rect, g: &Game) {
    let block = Block::bordered().title(" race ");
    let rows = Layout::vertical([Constraint::Length(1); 2]).split(block.inner(area));
    f.render_widget(block, area);
    for (row, p) in rows.iter().zip([Player::White, Player::Black]) {
        let pips = g.board.pips(p);
        let gauge = LineGauge::default()
            .ratio((1.0 - pips as f64 / START_PIPS).clamp(0.0, 1.0))
            .label(format!("{:<6}{pips:>4} ", format!("{p:?}")))
            .filled_style(Style::default().fg(style::player(p)));
        f.render_widget(gauge, *row);
    }
}
