mod board;
mod cell;
mod panel;
mod style;

use crate::app::Game;
use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn draw(f: &mut Frame, g: &Game) {
    let [left, right] = Layout::horizontal([Constraint::Length(50), Constraint::Min(30)]).areas(f.area());
    let [board, _] = Layout::vertical([Constraint::Length(15), Constraint::Min(0)]).areas(left);
    let [info, log, keys] =
        Layout::vertical([Constraint::Length(9), Constraint::Min(4), Constraint::Length(5)]).areas(right);
    f.render_widget(Paragraph::new(board::lines(g)).block(Block::bordered().title(" termgammon ")), board);
    f.render_widget(Paragraph::new(panel::info(g)).block(Block::bordered().title(" game ")), info);
    let shown = log.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = g.log.iter().rev().take(shown).rev().map(|s| Line::raw(s.clone())).collect();
    f.render_widget(Paragraph::new(lines).block(Block::bordered().title(" log ")), log);
    f.render_widget(Paragraph::new(panel::keys()).block(Block::bordered().title(" keys ")), keys);
}
