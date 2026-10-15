#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Ask {
    Quit,
    Leave,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Screen {
    Main,
    Setup,
    Playing,
    Pause,
    Settings,
    Help,
    Confirm(Ask),
    MatchOver,
}
