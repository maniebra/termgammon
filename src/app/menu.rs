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
    Yes,
    No,
    Back,
}

pub fn items(s: Screen, vs_ai: bool) -> &'static [Item] {
    use Item::*;
    match s {
        Screen::Main => &[VsAi, Hotseat, Settings, Help, Quit],
        Screen::Setup if vs_ai => &[Level, Side, Length, Start, Back],
        Screen::Setup => &[Length, Start, Back],
        Screen::Pause => &[Resume, NewGame, Settings, Help, MainMenu, Quit],
        Screen::Settings => &[Theme, Level, Side, Length, Back],
        Screen::Help => &[Back],
        Screen::Confirm(_) => &[No, Yes],
        Screen::MatchOver => &[Rematch, MainMenu],
        Screen::Playing => &[],
    }
}
