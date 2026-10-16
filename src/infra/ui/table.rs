use super::layout::{self, Fit};
use crate::app::Game;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    text::Line,
    widgets::Paragraph,
};

pub fn draw(f: &mut Frame, g: &Game) {
    let Some(board) = layout::board(f.area()) else {
        let (w, h) = (f.area().width, f.area().height);
        let text = format!("Terminal too small ({w}×{h}).\nNeed at least 50×19.\nEsc for menu.");
        return f.render_widget(Paragraph::new(text).centered(), f.area());
    };
    f.render_widget(Paragraph::new(super::board::lines(g)).block(super::style::block(" termgammon ")), board);
    match layout::fit(f.area()) {
        Fit::Full => side(f, g, board),
        _ => super::compact::draw(f, g, board),
    }
    super::fx::draw(f, g, board);
}

fn side(f: &mut Frame, g: &Game, board: Rect) {
    let [left, right] = layout::columns(f.area());
    let left = Rect { y: board.bottom(), height: left.bottom() - board.bottom(), ..left };
    let [race, log] = Layout::vertical([Constraint::Length(4), Constraint::Min(3)]).areas(left);
    let [info, keys, _] =
        Layout::vertical([Constraint::Length(14), Constraint::Length(5), Constraint::Min(0)]).areas(right);
    super::race::draw(f, race, g);
    f.render_widget(Paragraph::new(super::panel::info(g)).block(super::style::block(" game ")), info);
    let shown = log.height.saturating_sub(2) as usize;
    let lines: Vec<Line> = g.log.iter().rev().take(shown).rev().map(|s| Line::raw(s.clone())).collect();
    f.render_widget(Paragraph::new(lines).block(super::style::block(" log ")), log);
    f.render_widget(Paragraph::new(super::panel::keys()).block(super::style::block(" keys ")), keys);
}
