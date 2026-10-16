mod bright;
mod muted;

use crate::app::Theme;
use ratatui::style::Color;
use std::cell::Cell;

pub struct Palette {
    pub white: Color,
    pub black: Color,
    pub point: [Color; 2],
    pub select: [Color; 2],
    pub target: [Color; 2],
    pub last: Color,
    pub accent: Color,
    pub dim: Color,
    pub border: Color,
    pub backdrop: [Color; 3],
}

thread_local! {
    static CURRENT: Cell<Theme> = const { Cell::new(Theme::Classic) };
}

// ponytail: thread-local theme avoids threading the palette through every draw fn
pub fn set(t: Theme) {
    CURRENT.with(|c| c.set(t));
}

pub fn get() -> Palette {
    match CURRENT.with(Cell::get) {
        Theme::Classic => bright::classic(),
        Theme::Ocean => bright::ocean(),
        Theme::Neon => bright::neon(),
        Theme::Forest => muted::forest(),
        Theme::Mono => muted::mono(),
        Theme::Basic => muted::basic(),
    }
}
