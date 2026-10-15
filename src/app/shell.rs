use super::{Action, Game, Screen, menu::Item};
use crate::domain::Player;

pub struct App {
    pub game: Game,
    pub screen: Screen,
    pub index: usize,
    pub side: Player,
    pub quit: bool,
    back: Screen,
}

impl App {
    pub fn new() -> Self {
        let game = Game::new();
        Self { game, screen: Screen::Main, index: 0, side: Player::Black, quit: false, back: Screen::Main }
    }

    pub fn items(&self) -> &'static [Item] {
        super::menu::items(self.screen)
    }

    pub fn handle(&mut self, a: Action) {
        match (self.screen, a) {
            (Screen::Playing, Action::Cancel | Action::Quit) if self.game.selected.is_none() => self.open(Screen::Pause),
            (Screen::Playing, _) => self.game.handle(a),
            (_, Action::Quit) => self.quit = true,
            (_, Action::Cancel) => self.close(),
            (_, Action::Up) => self.index = (self.index + self.items().len() - 1) % self.items().len(),
            (_, Action::Down) => self.index = (self.index + 1) % self.items().len(),
            (_, Action::Left | Action::Right) => self.adjust(self.items()[self.index]),
            (_, Action::Enter) => self.pick(self.items()[self.index]),
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        if self.screen == Screen::Playing {
            self.game.tick();
        }
    }

    fn pick(&mut self, item: Item) {
        match item {
            Item::VsAi => self.start(Some(self.side)),
            Item::Hotseat => self.start(None),
            Item::NewGame => self.start(self.game.ai.map(|_| self.side)),
            Item::Resume => self.screen = Screen::Playing,
            Item::Settings => self.open(Screen::Settings),
            Item::Help => self.open(Screen::Help),
            Item::MainMenu => self.open(Screen::Main),
            Item::Quit => self.quit = true,
            Item::Level | Item::Side => self.adjust(item),
            Item::Back => self.close(),
        }
    }

    fn adjust(&mut self, item: Item) {
        match item {
            Item::Level => self.game.level = self.game.level.next(),
            Item::Side => self.side = self.side.other(),
            _ => {}
        }
    }

    fn start(&mut self, ai: Option<Player>) {
        self.game.set_ai(ai);
        self.game.new_match();
        self.screen = Screen::Playing;
    }

    fn open(&mut self, s: Screen) {
        if matches!(self.screen, Screen::Main | Screen::Pause) {
            self.back = self.screen;
        }
        self.screen = s;
        self.index = 0;
    }

    fn close(&mut self) {
        self.screen = match self.screen {
            Screen::Pause => Screen::Playing,
            Screen::Settings | Screen::Help => self.back,
            s => s,
        };
        self.index = 0;
    }
}
