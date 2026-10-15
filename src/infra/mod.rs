mod keys;
mod mouse;
mod names;
mod settings;
mod ui;

use crate::app::App;
use ratatui::crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyEventKind},
    execute,
};
use std::io::stdout;
use std::time::Duration;

pub fn run() -> std::io::Result<()> {
    let mut term = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
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
        match event::poll(Duration::from_millis(50)).and_then(|ready| ready.then(event::read).transpose()) {
            Ok(Some(Event::Key(k))) if k.kind == KeyEventKind::Press => {
                if let Some(a) = keys::action(k.code) {
                    app.handle(a);
                }
            }
            Ok(Some(Event::Mouse(m))) => {
                if let Some(a) = term.size().ok().and_then(|s| mouse::action(m, s.into())) {
                    app.handle(a);
                }
            }
            Ok(None) => app.tick(),
            Ok(_) => {}
            Err(e) => break Err(e),
        }
    };
    let _ = execute!(stdout(), DisableMouseCapture);
    ratatui::restore();
    settings::save(&app);
    res
}
