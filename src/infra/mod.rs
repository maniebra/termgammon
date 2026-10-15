mod keys;
mod names;
mod settings;
mod ui;

use crate::app::App;
use ratatui::crossterm::event::{self, Event, KeyEventKind};

pub fn run() -> std::io::Result<()> {
    let mut term = ratatui::init();
    let mut app = App::new();
    settings::load(&mut app);
    settings::args(&mut app, &std::env::args().skip(1).collect::<Vec<_>>());
    let res = loop {
        if app.quit {
            break Ok(());
        }
        if let Err(e) = term.draw(|f| ui::draw(f, &app)) {
            break Err(e);
        }
        match event::read() {
            Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => keys::action(k.code).map(|a| app.handle(a)),
            Ok(_) => None,
            Err(e) => break Err(e),
        };
    };
    ratatui::restore();
    settings::save(&app);
    res
}
