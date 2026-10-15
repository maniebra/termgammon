mod points;
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
    let lines = vec![
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
        Line::raw(format!(" Turn: {:?}   Dice: {:?}   AI: {:?}", g.turn, g.dice, g.level)),
        Line::raw(format!(" {}", g.msg)),
        Line::raw(""),
        Line::raw(" ←/→ move  Enter select/drop  Esc cancel  a ai level  q quit"),
    ];
    let log = g.log.iter().rev().take(4).rev().map(|s| Line::raw(format!(" {s}")));
    let lines: Vec<Line> = lines.into_iter().chain(log).collect();
    let block = Block::bordered().title(" termgammon ");
    f.render_widget(Paragraph::new(lines).block(block), f.area());
}
