use crate::app::Action;
use ratatui::crossterm::event::KeyCode;

pub fn action(key: KeyCode) -> Option<Action> {
    match key {
        KeyCode::Up | KeyCode::Char('k') => Some(Action::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(Action::Down),
        KeyCode::Left | KeyCode::Char('h') => Some(Action::Left),
        KeyCode::Right | KeyCode::Char('l') => Some(Action::Right),
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Enter),
        KeyCode::Esc => Some(Action::Cancel),
        KeyCode::Char('q') => Some(Action::Quit),
        KeyCode::Char('r') => Some(Action::Roll),
        KeyCode::Char('d') => Some(Action::Double),
        KeyCode::Char('y') => Some(Action::Take),
        KeyCode::Char('p') => Some(Action::Pass),
        _ => None,
    }
}
