use super::{geo, noise::put};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
};

const LIFE: u32 = 16;

pub fn draw(f: &mut Frame, board: Rect, at: usize, age: u32) {
    if age >= LIFE {
        return;
    }
    let (cx, cy) = geo::point(board, at, 0);
    let r = age as f32 * 0.45;
    let (glyph, color) = match age {
        0..4 => ("✦", Color::Yellow),
        4..9 => ("*", Color::LightRed),
        _ => ("·", Color::Red),
    };
    for k in 0..12 {
        let a = k as f32 * std::f32::consts::TAU / 12.0;
        put(f.buffer_mut(), board, cx + a.cos() * r * 2.0, cy + a.sin() * r, glyph, Style::default().fg(color));
    }
}
