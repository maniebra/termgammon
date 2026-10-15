mod keys;
mod names;
mod settings;
mod ui;

use crate::app::Game;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

pub fn run() -> std::io::Result<()> {
    let mut term = ratatui::init();
    let mut game = Game::new();
    settings::load(&mut game);
    settings::args(&mut game, &std::env::args().skip(1).collect::<Vec<_>>());
    let res = loop {
        if let Err(e) = term.draw(|f| ui::draw(f, &game)) {
            break Err(e);
        }
        let key = match event::read() {
            Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => k.code,
            Ok(_) => continue,
            Err(e) => break Err(e),
        };
        if key == KeyCode::Char('q') {
            break Ok(());
        }
        if let Some(a) = keys::action(key) {
            game.handle(a);
        }
    };
    ratatui::restore();
    settings::save(&game);
    res
}
