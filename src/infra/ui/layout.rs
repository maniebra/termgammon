use ratatui::layout::{Constraint, Flex, Layout, Rect};

pub const BOARD: (u16, u16) = (50, 15);
const FULL: (u16, u16) = (96, 22);
const COMPACT: (u16, u16) = (BOARD.0, BOARD.1 + 4);

#[derive(PartialEq)]
pub enum Fit {
    Full,
    Compact,
    Tiny,
}

pub fn fit(area: Rect) -> Fit {
    match (area.width, area.height) {
        (w, h) if w >= FULL.0 && h >= FULL.1 => Fit::Full,
        (w, h) if w >= COMPACT.0 && h >= COMPACT.1 => Fit::Compact,
        _ => Fit::Tiny,
    }
}

pub fn columns(area: Rect) -> [Rect; 2] {
    Layout::horizontal([Constraint::Length(BOARD.0), Constraint::Length(FULL.0 - BOARD.0)]).flex(Flex::Center).areas(area)
}

pub fn board(area: Rect) -> Option<Rect> {
    let column = match fit(area) {
        Fit::Full => columns(area)[0],
        Fit::Compact => Layout::horizontal([Constraint::Length(BOARD.0)]).flex(Flex::Center).areas::<1>(area)[0],
        Fit::Tiny => return None,
    };
    Some(Rect { height: BOARD.1, ..column })
}
