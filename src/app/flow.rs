use super::{App, Screen, menu::Item, screen::Ask};
use crate::domain::Player;

const LENGTHS: [u32; 5] = [1, 3, 5, 7, 9];

impl App {
    pub(super) fn pick(&mut self, item: Item) {
        match (item, self.screen) {
            (Item::VsAi | Item::Hotseat, _) => {
                self.vs_ai = item == Item::VsAi;
                self.open(Screen::Setup);
            }
            (Item::Start, _) => self.start(self.vs_ai.then_some(self.side)),
            (Item::NewGame | Item::Rematch, _) => self.start(self.game.ai.map(|_| self.side)),
            (Item::Resume, _) => self.screen = Screen::Playing,
            (Item::Settings, _) => self.open(Screen::Settings),
            (Item::Help, _) => self.open(Screen::Help),
            (Item::MainMenu, Screen::Pause) => self.open(Screen::Confirm(Ask::Leave)),
            (Item::Quit, Screen::Pause) => self.open(Screen::Confirm(Ask::Quit)),
            (Item::MainMenu | Item::Yes, Screen::Confirm(Ask::Leave) | Screen::MatchOver) => self.open(Screen::Main),
            (Item::Quit, _) | (Item::Yes, _) => self.quit = true,
            (Item::Level | Item::Side | Item::Length | Item::Theme | Item::Motion, _) => self.adjust(item),
            _ => self.close(),
        }
    }

    pub(super) fn adjust(&mut self, item: Item) {
        match item {
            Item::Level => self.game.level = self.game.level.next(),
            Item::Side => self.side = self.side.other(),
            Item::Theme => self.theme = self.theme.next(),
            Item::Motion => self.game.motion = !self.game.motion,
            Item::Length => {
                let i = LENGTHS.iter().position(|&n| n == self.game.target).unwrap_or(0);
                self.game.target = LENGTHS[(i + 1) % LENGTHS.len()];
            }
            _ => {}
        }
    }

    fn start(&mut self, ai: Option<Player>) {
        self.game.set_ai(ai);
        self.game.new_match();
        self.screen = Screen::Playing;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::Action::*;

    #[test]
    fn menu_flow() {
        let mut app = App::new();
        [Enter, Up, Up, Enter].into_iter().for_each(|a| app.handle(a));
        assert_eq!(app.screen, Screen::Playing);
        [Cancel, Quit, Enter].into_iter().for_each(|a| app.handle(a));
        assert_eq!((app.screen, app.quit), (Screen::Pause, false));
        [Quit, Down, Enter].into_iter().for_each(|a| app.handle(a));
        assert!(app.quit);
    }
}
