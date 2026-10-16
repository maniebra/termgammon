use super::Phase;
use crate::domain::{Board, Cube, Move, Player, ai::Level};
use crate::shared::Rng;

pub const OFF: usize = 24;

pub struct Game {
    pub board: Board,
    pub cube: Cube,
    pub phase: Phase,
    pub score: [u32; 2],
    pub target: u32,
    pub turn: Player,
    pub dice: Vec<u8>,
    pub rolled: Vec<u8>,
    pub last: Vec<Move>,
    pub anim: u8,
    pub cursor: usize,
    pub selected: Option<Option<usize>>,
    pub msg: String,
    pub log: Vec<String>,
    pub level: Level,
    pub ai: Option<Player>,
    pub ticks: u32,
    pub(super) wait: u8,
    pub(super) undo: Vec<(Board, Vec<u8>, Option<String>)>,
    pub(super) rng: Rng,
}

impl Game {
    pub fn new() -> Self {
        let mut g = Self {
            board: Board::default(),
            cube: Cube::default(),
            phase: Phase::Roll,
            score: [0; 2],
            target: 3,
            turn: Player::White,
            dice: vec![],
            rolled: vec![],
            last: vec![],
            anim: 0,
            cursor: 0,
            selected: None,
            msg: String::new(),
            log: vec![],
            level: Level::Medium,
            ai: Some(Player::Black),
            ticks: 0,
            wait: 0,
            undo: vec![],
            rng: Rng::new(),
        };
        g.reset();
        g
    }

    pub fn new_match(&mut self) {
        self.score = [0; 2];
        self.log.clear();
        self.reset();
    }

    pub(super) fn reset(&mut self) {
        self.board = Board::default();
        self.cube = Cube::default();
        self.selected = None;
        self.opening();
    }

    pub fn match_winner(&self) -> Option<Player> {
        [Player::White, Player::Black].into_iter().find(|p| self.score[p.idx()] >= self.target)
    }

    pub fn targets(&self) -> Vec<Option<usize>> {
        let from = self.source();
        self.legal().iter().filter(|m| Some(m.from) == from).map(|m| m.to).collect()
    }

    pub fn actor(&self) -> Player {
        if self.phase == Phase::Offer { self.turn.other() } else { self.turn }
    }

    pub(super) fn legal(&self) -> Vec<Move> {
        self.board.legal_moves(self.turn, &self.dice)
    }

    pub fn source(&self) -> Option<Option<usize>> {
        if self.board.bar[self.turn.idx()] > 0 { Some(None) } else { self.selected }
    }
}
