use crate::app::Action;
use ratatui::crossterm::event::KeyCode;

pub fn action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Left | KeyCode::Char('h') => Some(Action::Left),
        KeyCode::Right | KeyCode::Char('l') => Some(Action::Right),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Enter),
        KeyCode::Esc => Some(Action::Cancel),
        KeyCode::Char('a') => Some(Action::CycleLevel),
        KeyCode::Char('m') => Some(Action::CycleMode),
        KeyCode::Char('n') => Some(Action::NewGame),
        _ => None,
    }
}
