mod ui;

use crate::app::{Action, Game};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run() -> std::io::Result<()> {
    let mut term = ratatui::init();
    let mut game = Game::new();
    let res = loop {
        if let Err(e) = term.draw(|f| ui::draw(f, &game)) {
            break Err(e);
        }
        let key = match event::read() {
            Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => k.code,
            Ok(_) => continue,
            Err(e) => break Err(e),
        };
        let action = match key {
            KeyCode::Char('q') => break Ok(()),
            KeyCode::Left | KeyCode::Char('h') => Action::Left,
            KeyCode::Right | KeyCode::Char('l') => Action::Right,
            KeyCode::Enter | KeyCode::Char(' ') => Action::Enter,
            KeyCode::Esc => Action::Cancel,
            _ => continue,
        };
        game.handle(action);
    };
    ratatui::restore();
    res
}
