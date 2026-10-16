use super::{geo, noise::put};
use crate::app::{Game, SLIDE};
use crate::domain::Player;
use crate::infra::ui::style;
use ratatui::{Frame, layout::Rect, style::Style};

const TRAIL: [&str; 4] = ["●", "•", "∙", "·"];

pub fn draw(f: &mut Frame, g: &Game, board: Rect, from: Option<usize>, to: Option<usize>, p: Player, age: u32) {
    let (a, b) = (geo::source(g, from, p, board), geo::dest(g, to, p, board));
    let mid = board.y as f32 + 7.0;
    for (n, glyph) in TRAIL.iter().enumerate().rev() {
        let t = (age as f32 - n as f32 * 0.7) / SLIDE as f32;
        if !(0.0..=1.0).contains(&t) {
            continue;
        }
        let e = t * t * (3.0 - 2.0 * t);
        let y = a.1 + (b.1 - a.1) * e;
        let lift = (mid - y).signum() * (std::f32::consts::PI * e).sin() * 3.0;
        let color = if n == 0 { style::player(p) } else { style::glow(g.frame + n as u32 * 4) };
        put(f.buffer_mut(), board, a.0 + (b.0 - a.0) * e, y + lift, glyph, Style::default().fg(color));
    }
}
