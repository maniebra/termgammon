mod backdrop;
mod board;
mod cell;
mod dice;
mod fx;
mod help;
mod menu;
mod panel;
mod race;
mod style;
mod table;

pub use table::columns;

use crate::app::{App, Ask, Screen};
use ratatui::text::Line;
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
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
