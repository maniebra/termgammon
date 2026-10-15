use crate::app::Action;
use ratatui::crossterm::event::KeyCode;

pub fn action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Left | KeyCode::Char('h') => Some(Action::Left),
        KeyCode::Right | KeyCode::Char('l') => Some(Action::Right),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Enter),
        KeyCode::Esc => Some(Action::Cancel),
        _ => None,
    }
}
