mod points;
mod status;
mod style;

use crate::app::{Game, OFF};
use ratatui::{
    Frame,
    style::Color,
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn draw(f: &mut Frame, g: &Game) {
    let targets = g.targets();
    let b = &g.board;
    let top: Vec<usize> = (12..24).collect();
    let bottom: Vec<usize> = (0..12).rev().collect();
    let off = format!(" OFF  W:{}  B:{} ", b.off[0], b.off[1]);
    let mut lines = vec![
        points::labels(&top),
        points::cells(g, &top, &targets),
        Line::raw(""),
        Line::raw(format!(" BAR  W:{}  B:{}", b.bar[0], b.bar[1])),
        Line::raw(""),
        points::cells(g, &bottom, &targets),
        points::labels(&bottom),
        Line::raw(""),
        Line::from(Span::styled(off, style::of(g, OFF, &targets, Color::Reset))),
        Line::raw(""),
    ];
    lines.extend(status::lines(g));
    lines.extend(g.log.iter().rev().take(4).rev().map(|s| Line::raw(format!(" {s}"))));
    let block = Block::bordered().title(" termgammon ");
    f.render_widget(Paragraph::new(lines).block(block), f.area());
}
