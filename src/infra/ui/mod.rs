mod board;
mod cell;
mod dice;
mod help;
mod menu;
mod panel;
mod style;
mod table;

use crate::app::{App, Screen};
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
        Screen::Help => menu::popup(f, app, " how to play ", help::rules()),
    }
}
