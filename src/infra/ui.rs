use crate::app::{Game, OFF};
use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
};

pub fn draw(f: &mut Frame, g: &Game) {
    let targets = g.targets();
    let (b, w) = (&g.board, 0);
    let top: Vec<usize> = (12..24).collect();
    let bottom: Vec<usize> = (0..12).rev().collect();
    let off = format!(" OFF  W:{}  B:{} ", b.off[w], b.off[1]);
    let lines = vec![
        labels(&top),
        cells(g, &top, &targets),
        Line::raw(""),
        Line::raw(format!(" BAR  W:{}  B:{}", b.bar[w], b.bar[1])),
        Line::raw(""),
        cells(g, &bottom, &targets),
        labels(&bottom),
        Line::raw(""),
        Line::from(Span::styled(off, style(g, OFF, &targets, Color::Reset))),
        Line::raw(""),
        Line::raw(format!(" Turn: {:?}   Dice: {:?}", g.turn, g.dice)),
        Line::raw(format!(" {}", g.msg)),
        Line::raw(" ←/→ move  Enter select/drop  Esc cancel  q quit"),
    ];
    let block = Block::bordered().title(" termgammon ");
    f.render_widget(Paragraph::new(lines).block(block), f.area());
}

fn labels(ix: &[usize]) -> Line<'static> {
    Line::from(row(ix, |i| Span::raw(format!("{:>3} ", i + 1))))
}

fn cells<'a>(g: &Game, ix: &[usize], targets: &[Option<usize>]) -> Line<'a> {
    Line::from(row(ix, |i| {
        let v = g.board.points[i];
        let (text, fg) = match v {
            0 => ("  . ".to_string(), Color::DarkGray),
            v if v > 0 => (format!(" W{:<2}", v), Color::White),
            v => (format!(" B{:<2}", -v), Color::Red),
        };
        Span::styled(text, style(g, i, targets, fg))
    }))
}

fn row<'a>(ix: &[usize], cell: impl Fn(usize) -> Span<'a>) -> Vec<Span<'a>> {
    let mut spans: Vec<Span> = ix.iter().map(|&i| cell(i)).collect();
    spans.insert(6, Span::raw(" | "));
    spans
}

fn style(g: &Game, i: usize, targets: &[Option<usize>], fg: Color) -> Style {
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
