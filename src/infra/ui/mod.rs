mod board;
mod cell;
mod dice;
mod help;
mod menu;
mod panel;
mod race;
mod style;
mod table;

use crate::app::{App, Ask, Screen};
use ratatui::text::Line;
use ratatui::Frame;

pub fn draw(f: &mut Frame, app: &App) {
    match app.screen {
        Screen::Main => menu::popup(f, app, " termgammon ", help::banner()),
        Screen::Playing => table::draw(f, &app.game),
        Screen::Pause => {
            table::draw(f, &app.game);
            menu::popup(f, app, " paused ", vec![]);
        }
        Screen::Settings => menu::popup(f, app, " settings ", vec![]),
        Screen::Setup => menu::popup(f, app, if app.vs_ai { " vs ai " } else { " two players " }, vec![]),
        Screen::Confirm(ask) => {
            table::draw(f, &app.game);
            let text = if ask == Ask::Quit { "Quit termgammon?" } else { "Abandon this match?" };
            menu::popup(f, app, " confirm ", vec![Line::raw(text)]);
        }
        Screen::MatchOver => {
            table::draw(f, &app.game);
            menu::popup(f, app, " match over ", help::result(&app.game));
        }
        Screen::Help => menu::popup(f, app, " how to play ", help::rules()),
    }
}
