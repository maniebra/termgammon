use crate::app::Game;
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout, Rect},
    text::Line,
    widgets::{Block, Paragraph},
};

pub fn columns(area: Rect) -> [Rect; 2] {
    Layout::horizontal([Constraint::Length(50), Constraint::Length(46)]).flex(Flex::Center).areas(area)
}

pub fn draw(f: &mut Frame, g: &Game) {
    let [left, right] = columns(f.area());
    let [board, race, log] =
        Layout::vertical([Constraint::Length(15), Constraint::Length(4), Constraint::Min(3)]).areas(left);
    let [info, keys, _] =
        Layout::vertical([Constraint::Length(14), Constraint::Length(5), Constraint::Min(0)]).areas(right);
    f.render_widget(Paragraph::new(super::board::lines(g)).block(Block::bordered().title(" termgammon ")), board);
    super::race::draw(f, race, g);
    f.render_widget(Paragraph::new(super::panel::info(g)).block(Block::bordered().title(" game ")), info);
    let shown = log.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = g.log.iter().rev().take(shown).rev().map(|s| Line::raw(s.clone())).collect();
    f.render_widget(Paragraph::new(lines).block(Block::bordered().title(" log ")), log);
    f.render_widget(Paragraph::new(super::panel::keys()).block(Block::bordered().title(" keys ")), keys);
}
