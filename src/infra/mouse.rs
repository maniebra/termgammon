use super::ui;
use crate::app::{Action, OFF};
use ratatui::{
    crossterm::event::{MouseEvent, MouseEventKind},
    layout::Rect,
};

pub fn action(m: MouseEvent, screen: Rect) -> Option<Action> {
    if m.kind != MouseEventKind::Down(ratatui::crossterm::event::MouseButton::Left) {
        return None;
    }
    let board = ui::board(screen)?;
    let dx = m.column.checked_sub(board.x + 1)?;
    let dy = m.row.checked_sub(board.y + 1)?;
    let col = match dx {
        0..=17 => dx / 3,
        21..=38 => 6 + (dx - 21) / 3,
        42..=45 => return Some(Action::Click(OFF)),
        _ => return None,
    } as usize;
    match dy {
        0..=5 => Some(Action::Click(12 + col)),
        7..=12 => Some(Action::Click(11 - col)),
        _ => None,
    }
}
