mod backdrop;
mod board;
mod cell;
mod compact;
mod dice;
mod fx;
mod help;
mod layout;
mod menu;
mod panel;
mod race;
mod style;
mod table;
mod theme;

pub use layout::board;

use crate::app::{App, Ask, Screen};
use ratatui::Frame;
use ratatui::text::Line;

pub fn draw(f: &mut Frame, app: &App) {
    theme::set(app.theme);
    match app.screen {
        Screen::Main => {
            backdrop::draw(f, app.game.frame);
            menu::popup(f, app, " termgammon ", help::banner(app.game.frame));
        }
        Screen::Playing => table::draw(f, &app.game),
        Screen::Pause => {
            table::draw(f, &app.game);
            menu::popup(f, app, " paused ", vec![]);
        }
        Screen::Settings | Screen::Setup | Screen::Help => {
            backdrop::draw(f, app.game.frame);
            let title = match app.screen {
                Screen::Settings => " settings ",
                Screen::Help => " how to play ",
                _ if app.vs_ai => " vs ai ",
                _ => " two players ",
            };
            let text = if app.screen == Screen::Help { help::rules() } else { vec![] };
            menu::popup(f, app, title, text);
        }
        Screen::Confirm(ask) => {
            table::draw(f, &app.game);
            let text = if ask == Ask::Quit { "Quit termgammon?" } else { "Abandon this match?" };
            menu::popup(f, app, " confirm ", vec![Line::raw(text)]);
        }
        Screen::MatchOver => {
            table::draw(f, &app.game);
            menu::popup(f, app, " match over ", help::result(&app.game));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Action::*;

    #[test]
    fn renders_at_any_size() {
        let mut app = App::new();
        for step in [vec![], vec![Enter], vec![Up, Up, Enter], vec![Roll, Enter, Enter], vec![Cancel, Down, Down]] {
            step.into_iter().for_each(|a| app.handle(a));
            (0..30).for_each(|_| app.tick());
            for (w, h) in [(1, 1), (20, 8), (50, 19), (60, 22), (96, 22), (160, 50)] {
                let mut t = ratatui::Terminal::new(ratatui::backend::TestBackend::new(w, h)).unwrap();
                t.draw(|f| draw(f, &app)).unwrap();
            }
        }
    }
}
