use super::Action;
use crate::domain::{Board, Move, Player, ai::Level};
use crate::shared::Rng;

pub const OFF: usize = 24;

pub struct Game {
    pub board: Board,
    pub turn: Player,
    pub dice: Vec<u8>,
    pub cursor: usize,
    pub selected: Option<Option<usize>>,
    pub msg: String,
    pub log: Vec<String>,
    pub level: Level,
    pub ai: Option<Player>,
    pub(super) rng: Rng,
}

impl Game {
    pub fn new() -> Self {
        let mut g = Self {
            board: Board::default(),
            turn: Player::White,
            dice: vec![],
            cursor: 0,
            selected: None,
            msg: String::new(),
            log: vec![],
            level: Level::Medium,
            ai: Some(Player::Black),
            rng: Rng::new(),
        };
        g.roll();
        g.advance();
        g
    }

    pub fn targets(&self) -> Vec<Option<usize>> {
        let from = self.source();
        self.legal().iter().filter(|m| Some(m.from) == from).map(|m| m.to).collect()
    }

    pub fn handle(&mut self, a: Action) {
        match a {
            Action::CycleLevel => return self.level = self.level.next(),
            Action::CycleMode => return self.cycle_mode(),
            _ if self.board.winner().is_some() => return,
            _ => {}
        }
        match a {
            Action::Left => self.cursor = (self.cursor + OFF) % (OFF + 1),
            Action::Right => self.cursor = (self.cursor + 1) % (OFF + 1),
            Action::Cancel => self.selected = None,
            Action::Enter => self.enter(),
            _ => {}
        }
    }

    fn cycle_mode(&mut self) {
        self.ai = match self.ai {
            Some(Player::Black) => Some(Player::White),
            Some(Player::White) => None,
            None => Some(Player::Black),
        };
        self.selected = None;
        self.advance();
    }

    pub(super) fn legal(&self) -> Vec<Move> {
        self.board.legal_moves(self.turn, &self.dice)
    }

    fn source(&self) -> Option<Option<usize>> {
        if self.board.bar[self.turn.idx()] > 0 { Some(None) } else { self.selected }
    }

    fn enter(&mut self) {
        let at = (self.cursor < OFF).then_some(self.cursor);
        let legal = self.legal();
        let Some(from) = self.source() else {
            match legal.iter().any(|m| m.from == at) {
                true => self.selected = Some(at),
                false => self.msg = "No move from there".into(),
            }
            return;
        };
        self.selected = None;
        match legal.into_iter().find(|m| m.from == from && m.to == at) {
            Some(m) => {
                self.msg.clear();
                self.play(m);
                self.advance();
            }
            None => self.msg = "Illegal move".into(),
        }
    }
}
