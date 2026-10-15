use super::{Game, Phase};
use crate::domain::{Player, ai};

impl Game {
    pub(super) fn offer(&mut self) {
        let taker = self.turn.other();
        self.log.push(format!("{:?} doubles to {}", self.turn, self.cube.value * 2));
        match self.ai == Some(taker) {
            true if ai::takes(&self.board, taker) => self.accept(),
            true => self.decline(),
            false => self.phase = Phase::Offer,
        }
    }

    pub(super) fn accept(&mut self) {
        self.cube.take(self.turn.other());
        self.note(", taken");
        self.phase = Phase::Roll;
    }

    pub(super) fn decline(&mut self) {
        self.note(", passed");
        self.finish(self.turn, self.cube.value);
    }

    pub(super) fn finish(&mut self, winner: Player, points: u32) {
        self.score[winner.idx()] += points;
        self.phase = Phase::Over;
        self.log.push(format!("{winner:?} wins {points} point(s). Enter for next game"));
    }
}
