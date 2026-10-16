use super::Palette;
use ratatui::style::Color::*;

pub fn forest() -> Palette {
    Palette {
        white: Rgb(240, 230, 200),
        black: Rgb(200, 70, 40),
        point: [Rgb(45, 95, 45), Rgb(25, 60, 25)],
        select: [Rgb(230, 200, 80), Rgb(255, 160, 40)],
        target: [Rgb(120, 180, 60), Rgb(190, 240, 110)],
        last: Rgb(110, 80, 40),
        accent: Rgb(230, 200, 90),
        dim: Rgb(110, 130, 90),
        border: Rgb(100, 140, 80),
        backdrop: [Rgb(50, 80, 50), Rgb(100, 50, 30), Rgb(60, 70, 50)],
    }
}

pub fn mono() -> Palette {
    Palette {
        white: White,
        black: Rgb(140, 140, 140),
        point: [Rgb(60, 60, 60), Rgb(35, 35, 35)],
        select: [Rgb(230, 230, 230), Rgb(170, 170, 170)],
        target: [Rgb(110, 110, 110), Rgb(190, 190, 190)],
        last: Rgb(80, 80, 80),
        accent: White,
        dim: DarkGray,
        border: Gray,
        backdrop: [Rgb(70, 70, 70), Rgb(50, 50, 50), Rgb(40, 40, 40)],
    }
}

pub fn basic() -> Palette {
    Palette {
        white: White,
        black: Red,
        point: [DarkGray, Black],
        select: [Yellow, Yellow],
        target: [Green, Green],
        last: Blue,
        accent: Yellow,
        dim: DarkGray,
        border: Reset,
        backdrop: [DarkGray, Red, DarkGray],
    }
}
