use super::Screen;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Item {
    VsAi,
    Hotseat,
    Start,
    Resume,
    NewGame,
    Rematch,
    Settings,
    Help,
    MainMenu,
    Quit,
    Level,
    Side,
    Length,
    Theme,
    Motion,
    Yes,
    Resign(u32),
    Forfeit,
    No,
    Back,
}

pub fn items(s: Screen, vs_ai: bool) -> &'static [Item] {
    use Item::*;
    match s {
        Screen::Main => &[VsAi, Hotseat, Settings, Help, Quit],
        Screen::Setup if vs_ai => &[Level, Side, Length, Start, Back],
        Screen::Setup => &[Length, Start, Back],
        Screen::Pause => &[Resume, Forfeit, NewGame, Settings, Help, MainMenu, Quit],
        Screen::Settings => &[Theme, Motion, Level, Side, Length, Back],
        Screen::Help => &[Back],
        Screen::Confirm(_) => &[No, Yes],
        Screen::MatchOver => &[Rematch, MainMenu],
        Screen::Resign => &[No, Resign(1), Resign(2), Resign(3)],
        Screen::Playing => &[],
    }
}
