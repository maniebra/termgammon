use crate::app::{App, Item};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, Clear, Paragraph},
};

pub fn popup(f: &mut Frame, app: &App, title: &str, mut lines: Vec<Line<'static>>) {
    if !lines.is_empty() {
        lines.push(Line::raw(""));
    }
    lines.extend(app.items().iter().enumerate().map(|(i, &item)| entry(app, item, i == app.index)));
    let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16 + 6;
    let [area] = Layout::vertical([Constraint::Length(lines.len() as u16 + 4)]).flex(Flex::Center).areas(f.area());
    let [area] = Layout::horizontal([Constraint::Length(width.max(30))]).flex(Flex::Center).areas(area);
    let block = Block::bordered().title(title.to_string()).padding(ratatui::widgets::Padding::uniform(1));
    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(lines).block(block).centered(), area);
}

fn entry(app: &App, item: Item, selected: bool) -> Line<'static> {
    let text = format!("{} {}", if selected { "▶" } else { " " }, label(app, item));
    let style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    if selected { Line::styled(text, style) } else { Line::raw(text) }
}

fn label(app: &App, item: Item) -> String {
    match item {
        Item::VsAi => "Play vs AI".into(),
        Item::Hotseat => "Two players".into(),
        Item::Resume => "Resume".into(),
        Item::NewGame => "New match".into(),
        Item::Settings => "Settings".into(),
        Item::Help => "How to play".into(),
        Item::MainMenu => "Main menu".into(),
        Item::Quit => "Quit".into(),
        Item::Level => format!("AI level  ◀ {:?} ▶", app.game.level),
        Item::Side => format!("AI plays  ◀ {:?} ▶", app.side),
        Item::Length => format!("Match to  ◀ {} ▶", app.game.target),
        Item::Start => "Start".into(),
        Item::Rematch => "Rematch".into(),
        Item::Yes => "Yes".into(),
        Item::No => "No".into(),
        Item::Back => "Back".into(),
    }
}
