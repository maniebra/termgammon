use super::Game;
use crate::domain::Player;

pub const SLIDE: u32 = 10;
const LIFETIME: u32 = 150;

#[derive(Clone, PartialEq, Debug)]
pub enum Fx {
    Slide { from: Option<usize>, to: Option<usize>, player: Player },
    Burst(usize),
    Toast(String),
    Confetti,
}

pub struct Effect {
    pub fx: Fx,
    pub start: u32,
}

impl Game {
    pub(super) fn fx(&mut self, fx: Fx) {
        if !self.motion {
            return;
        }
        self.effects.push(Effect { fx, start: self.frame });
    }

    pub fn age(&self, e: &Effect) -> u32 {
        self.frame.wrapping_sub(e.start)
    }

    pub fn sliding_into(&self, to: Option<usize>) -> bool {
        self.effects.iter().any(|e| matches!(e.fx, Fx::Slide { to: t, .. } if t == to) && self.age(e) < SLIDE)
    }

    pub fn clock(&self) -> u32 {
        if self.motion { self.frame } else { 0 }
    }

    pub fn animate(&mut self) {
        self.frame = self.frame.wrapping_add(1);
        let frame = self.frame;
        self.effects.retain(|e| frame.wrapping_sub(e.start) < LIFETIME);
    }
}
