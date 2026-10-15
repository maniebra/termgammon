use crate::app::{Game, OFF};
use ratatui::style::{Color, Modifier, Style};

pub fn of(g: &Game, i: usize, targets: &[Option<usize>], fg: Color) -> Style {
    let pos = (i < OFF).then_some(i);
    let mut s = Style::default().fg(fg);
    if g.selected == Some(pos) {
        s = s.bg(Color::Yellow).fg(Color::Black);
    } else if targets.contains(&pos) {
        s = s.bg(Color::Green).fg(Color::Black);
    }
    if g.cursor == i {
        s = s.add_modifier(Modifier::REVERSED);
    }
    s
}
