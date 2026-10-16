mod burst;
mod confetti;
mod geo;
mod noise;
mod slide;
mod toast;

pub use noise::hash;

use crate::app::{Fx, Game};
use ratatui::{Frame, layout::Rect};

pub fn draw(f: &mut Frame, g: &Game, board: Rect) {
    for e in &g.effects {
        let age = g.age(e);
        match &e.fx {
            Fx::Slide { from, to, player } if age < crate::app::SLIDE => slide::draw(f, g, board, *from, *to, *player, age),
            Fx::Burst(at) => burst::draw(f, board, *at, age),
            Fx::Confetti => confetti::draw(f, age),
            _ => {}
        }
    }
    let toast = g.effects.iter().rev().find_map(|e| if let Fx::Toast(t) = &e.fx { Some((t, g.age(e))) } else { None });
    if let Some((text, age)) = toast {
        toast::draw(f, board, text, age, g.frame);
    }
}
