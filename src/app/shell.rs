use super::{Action, Game, Phase, Screen, menu::Item};
use crate::domain::Player;

pub struct App {
    pub game: Game,
    pub screen: Screen,
    pub index: usize,
    pub side: Player,
    pub vs_ai: bool,
    pub quit: bool,
    pub(super) back: Screen,
}

impl App {
    pub fn new() -> Self {
        let game = Game::new();
        let (screen, side) = (Screen::Main, Player::Black);
        Self { game, screen, index: 0, side, vs_ai: true, quit: false, back: screen }
    }

    pub fn items(&self) -> &'static [Item] {
        super::menu::items(self.screen, self.vs_ai)
    }

    pub fn handle(&mut self, a: Action) {
        match (self.screen, a) {
            (Screen::Playing, Action::Cancel | Action::Quit) if self.game.selected.is_none() => self.open(Screen::Pause),
            (Screen::Playing, _) => self.game.handle(a),
            (_, Action::Quit) => self.pick(Item::Quit),
            (_, Action::Cancel) => self.close(),
            (_, Action::Up) => self.index = (self.index + self.items().len() - 1) % self.items().len(),
            (_, Action::Down) => self.index = (self.index + 1) % self.items().len(),
            (_, Action::Left | Action::Right) => self.adjust(self.items()[self.index]),
            (_, Action::Enter) => self.pick(self.items()[self.index]),
            _ => {}
        }
        self.sync();
    }

    pub fn tick(&mut self) {
        if self.screen == Screen::Playing {
            self.game.tick();
            self.sync();
        }
    }

    fn sync(&mut self) {
        if self.screen == Screen::Playing && self.game.phase == Phase::Over && self.game.match_winner().is_some() {
            self.open(Screen::MatchOver);
        }
    }

    pub(super) fn open(&mut self, s: Screen) {
        if !matches!(s, Screen::Playing | Screen::Main | Screen::MatchOver) {
            self.back = self.screen;
        }
        self.screen = s;
        self.index = 0;
    }

    pub(super) fn close(&mut self) {
        self.screen = match self.screen {
            Screen::Pause => Screen::Playing,
            Screen::Setup | Screen::Settings | Screen::Help | Screen::Confirm(_) => self.back,
            _ => Screen::Main,
        };
        self.index = 0;
    }
}
