use crate::app::{App, Item};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    style::{Modifier, Style},
    text::Line,
    widgets::{Clear, Paragraph},
};

pub fn popup(f: &mut Frame, app: &App, title: &str, mut lines: Vec<Line<'static>>) {
    lines = lines.into_iter().map(Line::centered).collect();
    if !lines.is_empty() {
        lines.push(Line::raw(""));
    }
    lines.extend(app.items().iter().enumerate().map(|(i, &item)| entry(app, item, i == app.index)));
    let width = lines.iter().map(|l| l.width()).max().unwrap_or(0) as u16 + 6;
    let grow = if app.game.motion { app.game.frame.wrapping_sub(app.opened).min(8) as u16 } else { 8 };
    let height = (lines.len() as u16 + 4).min(3 + grow * 3);
    let width = width.max(30).min(10 + grow * 8);
    let [area] = Layout::vertical([Constraint::Length(height)]).flex(Flex::Center).areas(f.area());
    let [area] = Layout::horizontal([Constraint::Length(width)]).flex(Flex::Center).areas(area);
    let block = super::style::block(title).padding(ratatui::widgets::Padding::uniform(1));
    f.render_widget(Clear, area);
    f.render_widget(Paragraph::new(lines).block(block), area);
}

fn entry(app: &App, item: Item, selected: bool) -> Line<'static> {
    let text = format!("{} {}", if selected { "▶" } else { " " }, label(app, item));
    let style = Style::default().fg(super::style::accent()).add_modifier(Modifier::BOLD);
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
        Item::Theme => format!("Theme     ◀ {:?} ▶", app.theme),
        Item::Motion => format!("Animations  ◀ {} ▶", if app.game.motion { "On" } else { "Off" }),
        Item::Forfeit => "Resign".into(),
        Item::Resign(n) => ["", "Resign a single game", "Resign a gammon", "Resign a backgammon"][n as usize].into(),
        Item::Start => "Start".into(),
        Item::Rematch => "Rematch".into(),
        Item::Yes => "Yes".into(),
        Item::No => "No".into(),
        Item::Back => "Back".into(),
    }
}
