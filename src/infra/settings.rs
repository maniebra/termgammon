use super::names;
use crate::app::Game;
use std::{fs, path::PathBuf};

fn path() -> Option<PathBuf> {
    let base = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".config")))?;
    Some(base.join("termgammon").join("settings"))
}

pub fn load(g: &mut Game) {
    let text = path().and_then(|p| fs::read_to_string(p).ok()).unwrap_or_default();
    apply(g, text.lines().filter_map(|l| l.split_once('=')));
}

pub fn args(g: &mut Game, args: &[String]) {
    apply(g, args.chunks(2).filter_map(|c| Some((c[0].strip_prefix("--")?, c.get(1)?.as_str()))));
}

fn apply<'a>(g: &mut Game, pairs: impl Iterator<Item = (&'a str, &'a str)>) {
    for (key, value) in pairs {
        match key {
            "level" => g.level = names::level(value).unwrap_or(g.level),
            "ai" => g.set_ai(names::ai(value).unwrap_or(g.ai)),
            _ => {}
        }
    }
}

pub fn save(g: &Game) {
    let Some(p) = path() else { return };
    let text = format!("level={}\nai={}\n", format!("{:?}", g.level).to_lowercase(), names::ai_name(g.ai));
    let _ = p.parent().map(fs::create_dir_all);
    let _ = fs::write(p, text);
}
