use crate::app::{Game, OFF};
use crate::domain::Player;
use ratatui::style::{Color, Modifier, Style};

pub fn player(p: Player) -> Color {
    if p == Player::White { Color::White } else { Color::LightRed }
}

pub fn point_bg(i: usize) -> Color {
    if i.is_multiple_of(2) { Color::Rgb(120, 80, 50) } else { Color::Rgb(70, 50, 35) }
}

pub fn mark(g: &Game, i: usize, targets: &[Option<usize>], base: Style) -> Style {
    let pos = (i < OFF).then_some(i);
    let mut s = base;
    if g.selected == Some(pos) {
        s = s.bg(Color::Yellow).fg(Color::Black);
    } else if targets.contains(&pos) {
        s = s.bg(Color::Green).fg(Color::Black);
    }
    if g.cursor == i {
        s = s.add_modifier(Modifier::REVERSED | Modifier::BOLD);
    }
    s
}
