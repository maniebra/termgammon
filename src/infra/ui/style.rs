use super::theme;
use crate::app::{Game, OFF};
use crate::domain::Player;
use ratatui::{
    style::{Color, Modifier, Style},
    widgets::Block,
};

pub fn player(p: Player) -> Color {
    let t = theme::get();
    if p == Player::White { t.white } else { t.black }
}

pub fn accent() -> Color {
    theme::get().accent
}

pub fn dim() -> Color {
    theme::get().dim
}

pub fn block(title: &str) -> Block<'static> {
    Block::bordered().title(title.to_string()).border_style(Style::default().fg(theme::get().border))
}

pub fn glow(t: u32) -> Color {
    let h = (t % 360) as f32 / 60.0;
    let x = (1.0 - (h % 2.0 - 1.0).abs()) * 255.0;
    let (r, g, b) = match h as u32 {
        0 => (255.0, x, 0.0),
        1 => (x, 255.0, 0.0),
        2 => (0.0, 255.0, x),
        3 => (0.0, x, 255.0),
        4 => (x, 0.0, 255.0),
        _ => (255.0, 0.0, x),
    };
    Color::Rgb(r as u8, g as u8, b as u8)
}

fn pulse(frame: u32, [a, b]: [Color; 2]) -> Color {
    let t = ((frame as f32 * 0.2).sin() + 1.0) / 2.0;
    let mix = |x: u8, y: u8| (x as f32 + (y as f32 - x as f32) * t) as u8;
    match (a, b) {
        (Color::Rgb(r1, g1, b1), Color::Rgb(r2, g2, b2)) => Color::Rgb(mix(r1, r2), mix(g1, g2), mix(b1, b2)),
        _ => {
            if t < 0.5 {
                a
            } else {
                b
            }
        }
    }
}

pub fn point_bg(i: usize) -> Color {
    theme::get().point[i % 2]
}

pub fn mark(g: &Game, i: usize, targets: &[Option<usize>], base: Style) -> Style {
    let pos = (i < OFF).then_some(i);
    let (mut s, t) = (base, theme::get());
    if g.selected == Some(pos) {
        s = s.bg(pulse(g.clock(), t.select)).fg(Color::Black);
    } else if targets.contains(&pos) {
        s = s.bg(pulse(g.clock(), t.target)).fg(Color::Black);
    } else if g.last.iter().any(|m| m.to == pos || (m.from == pos && pos.is_some())) {
        s = s.bg(t.last);
    }
    if g.cursor == i {
        s = s.add_modifier(Modifier::REVERSED | Modifier::BOLD);
    }
    s
}
