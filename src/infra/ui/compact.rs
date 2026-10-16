use super::{panel, style};
use crate::app::Game;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub fn draw(f: &mut Frame, g: &Game, board: Rect) {
    let rest = Rect { y: board.bottom(), height: f.area().bottom() - board.bottom(), ..board };
    let [status, log, keys] = Layout::vertical([Constraint::Length(2), Constraint::Min(1), Constraint::Length(1)]).areas(rest);
    let dice: String = g.rolled.iter().map(|&d| format!("{} ", char::from_u32(0x267f + d as u32).unwrap())).collect();
    let turn = Span::styled(format!("● {:?}", g.turn), Style::default().fg(style::player(g.turn)));
    let info = format!("  {dice}  cube {}  score {}-{} /{}", g.cube.value, g.score[0], g.score[1], g.target);
    let prompt = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let lines = vec![Line::from(vec![turn, Span::raw(info)]), Line::styled(panel::prompt(g), prompt)];
    f.render_widget(Paragraph::new(lines), status);
    let shown = log.height as usize;
    let lines: Vec<Line> = g.log.iter().rev().take(shown).rev().map(|s| Line::raw(s.clone())).collect();
    f.render_widget(Paragraph::new(lines).style(Style::default().fg(Color::DarkGray)), log);
    let hint = "arrows tab enter · u undo · r roll · d double · esc";
    f.render_widget(Paragraph::new(hint).style(Style::default().fg(Color::DarkGray)), keys);
}
