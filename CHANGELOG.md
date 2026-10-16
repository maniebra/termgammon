# Changelog

All notable changes to termgammon are listed here.
The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

### Gameplay
- Full backgammon rules: hitting, the bar, bearing off, and using as many dice as possible (the larger die when only one can be played)
- Opening roll decides who starts
- Doubling cube with take / pass, gammons and backgammons
- Matches to 1, 3, 5, 7 or 9 points, with the Crawford rule
- Resign a single game, a gammon or a backgammon; the AI may decline
- Two-player hotseat mode

### AI
- Three levels: Easy (random), Medium (greedy) and Hard (searches every way to play the roll)
- AI doubles, takes and passes on its own
- Hint key (`?`) shows the Hard AI's suggested move

### Controls
- Screen-based cursor: arrows follow the board, up and down switch rows
- Tab jumps to the next playable checker or destination
- Mouse support: click to pick up and drop checkers
- One-press moves when a checker has a single destination
- Forced moves play themselves
- Undo within a turn

### Look and feel
- Board with stacked checkers, highlighted legal moves and last move
- Dice drawn with pips, tumbling when rolled
- Sliding checkers, hit sparks, pop-up banners and confetti on wins
- Animated menus with a rainbow title and falling checkers
- Race panel with pip counts, pip lead and win chance
- Six themes: Classic, Ocean, Neon, Forest, Mono and Basic (16 colours)
- Compact layout for terminals down to 50×19
- Option to turn animations off

### Menus
- Main, setup, pause, settings, help, confirm, resign and match-over screens
- Settings saved between sessions; command-line flags `--level`, `--side`, `--length`, `--theme`, `--animations`, `--seed`

### Distribution
- Release builds for Linux, macOS and Windows on x86_64 and arm64
- `.deb`, `.rpm` and AppImage packages, and publishing to crates.io
- Version is taken from the release tag
- `make install` for building from source
