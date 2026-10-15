use crate::domain::{Board, Move, Player};
use crate::shared::Rng;

pub enum Action {
    Left,
    Right,
    Enter,
    Cancel,
}

pub const OFF: usize = 24;

pub struct Game {
    pub board: Board,
    pub turn: Player,
    pub dice: Vec<u8>,
    pub cursor: usize,
    pub selected: Option<Option<usize>>,
    pub msg: String,
    rng: Rng,
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
        if self.board.winner().is_some() {
            return;
        }
        match a {
            Action::Left => self.cursor = (self.cursor + OFF) % (OFF + 1),
            Action::Right => self.cursor = (self.cursor + 1) % (OFF + 1),
            Action::Cancel => self.selected = None,
            Action::Enter => self.enter(),
        }
    }

    fn legal(&self) -> Vec<Move> {
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
            Some(m) => self.play(m),
            None => self.msg = "Illegal move".into(),
        }
    }

    fn play(&mut self, m: Move) {
        self.msg.clear();
        self.board.apply(self.turn, m);
        let i = self.dice.iter().position(|&d| d == m.die).unwrap();
        self.dice.remove(i);
        self.advance();
    }

    fn roll(&mut self) {
        let (a, b) = (self.rng.die(), self.rng.die());
        self.dice = if a == b { vec![a; 4] } else { vec![a, b] };
    }

    fn advance(&mut self) {
        if let Some(p) = self.board.winner() {
            self.msg = format!("{p:?} wins! Press q to quit");
            return;
        }
        while self.legal().is_empty() {
            if !self.dice.is_empty() {
                self.msg = format!("{:?} rolled {:?}, no moves", self.turn, self.dice);
            }
            self.turn = self.turn.other();
            self.roll();
        }
    }
}
