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
use std::time::{Duration, Instant};

const FRAME: Duration = Duration::from_millis(30);

pub fn run() -> std::io::Result<()> {
    let mut term = ratatui::init();
    execute!(stdout(), EnableMouseCapture)?;
    let mut app = App::new();
    settings::load(&mut app);
    settings::args(&mut app, &std::env::args().skip(1).collect::<Vec<_>>());
    let mut last = Instant::now();
    let res = loop {
        if app.quit {
            break Ok(());
        }
        if let Err(e) = term.draw(|f| ui::draw(f, &app)) {
            break Err(e);
        }
        if last.elapsed() >= FRAME {
            last = Instant::now();
            app.tick();
        }
        match event::poll(FRAME.saturating_sub(last.elapsed())).and_then(|ready| ready.then(event::read).transpose()) {
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
            Ok(_) => {}
            Err(e) => break Err(e),
        }
    };
    let _ = execute!(stdout(), DisableMouseCapture);
    ratatui::restore();
    settings::save(&app);
    res
}
