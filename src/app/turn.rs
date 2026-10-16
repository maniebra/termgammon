use super::{Fx, Game, Phase};
use crate::domain::{Move, Player, without};

const ROLL_FRAMES: u8 = 16;

impl Game {
    pub(super) fn play(&mut self, m: Move) {
        let bar = self.board.bar[self.turn.other().idx()];
        self.board.apply(self.turn, m);
        self.last.push(m);
        self.dice = without(&self.dice, m.die);
        self.fx(Fx::Slide { from: m.from, to: m.to, player: self.turn });
        let hit = if self.board.bar[self.turn.other().idx()] > bar { "*" } else { "" };
        if let (Some(at), "*") = (m.to, hit) {
            self.fx(Fx::Burst(at));
            self.fx(Fx::Toast("HIT!".into()));
        }
        self.note(&format!(" {}/{}{hit}", pos(m.from, "bar"), pos(m.to, "off")));
    }

    pub(super) fn roll(&mut self) {
        let (a, b) = (self.rng.die(), self.rng.die());
        self.dice = if a == b { vec![a; 4] } else { vec![a, b] };
        if a == b {
            self.fx(Fx::Toast("DOUBLES!".into()));
        }
        self.show(a, b);
        self.phase = Phase::Move;
        self.log.push(format!("{:?} {a}-{b}:", self.turn));
    }

    pub(super) fn opening(&mut self) {
        let (w, b) = loop {
            let (w, b) = (self.rng.die(), self.rng.die());
            if w != b {
                break (w, b);
            }
        };
        self.turn = if w > b { Player::White } else { Player::Black };
        self.dice = vec![w, b];
        self.show(w, b);
        self.phase = Phase::Move;
        self.log.push(format!("Opening W{w} B{b}, {:?}:", self.turn));
    }

    pub(super) fn advance(&mut self) {
        loop {
            match self.phase {
                Phase::Move if self.board.winner().is_some() => {
                    let w = self.board.winner().unwrap();
                    let kind = ["", "WINS!", "GAMMON!", "BACKGAMMON!"][self.board.multiplier(w) as usize];
                    self.fx(Fx::Toast(format!("{w:?} {kind}").to_uppercase()));
                    return self.finish(w, self.cube.value * self.board.multiplier(w));
                }
                Phase::Move if self.legal().is_empty() => self.end_turn(),
                _ => return self.aim(),
            }
        }
    }

    fn end_turn(&mut self) {
        if !self.dice.is_empty() {
            self.note(" no moves");
        }
        self.dice.clear();
        self.undo.clear();
        self.selected = None;
        self.turn = self.turn.other();
        self.phase = Phase::Roll;
    }

    fn show(&mut self, a: u8, b: u8) {
        self.rolled = vec![a, b];
        self.last.clear();
        self.anim = ROLL_FRAMES;
    }

    pub(super) fn note(&mut self, s: &str) {
        if let Some(last) = self.log.last_mut() {
            last.push_str(s);
        }
    }
}

fn pos(p: Option<usize>, edge: &str) -> String {
    p.map_or(edge.into(), |i| (i + 1).to_string())
}
