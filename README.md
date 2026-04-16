<div align="center">
  <h1>🎮 Timepour</h1>
  <p><strong>An arcade-style terminal focus timer where passing time fills the screen with a deterministic tetromino stack.</strong></p>

  [![License](https://img.shields.io/github/license/Hyena0x/timepour)](https://github.com/Hyena0x/timepour/blob/main/LICENSE)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/Hyena0x/timepour/rust.yml)](https://github.com/Hyena0x/timepour/actions)
</div>

<br />

Timepour turns a plain countdown into something you can feel.

Instead of staring at shrinking digits, you watch time accumulate as a clean, deterministic stack of falling tetrominoes. Focus sessions glow hot. Break sessions cool down. When the timer ends, the board dies into grayscale and the run is over.

It is a small terminal tool, but it feels more like a tiny arcade cabinet for your next focus block.

## Why it stands out

- Visual by default: time is represented as motion and density, not just numbers.
- Deterministic feel: the stack progression is consistent and intentional instead of noisy or random.
- Terminal-native: fast startup, no account, no browser tab, no desktop bloat.
- Session mood shift: focus and break modes use different color themes.
- Built for presence: the timer is harder to ignore than a quiet menu bar app.

## What you get

- A full-screen TUI countdown experience
- Animated tetromino stacking tied to countdown progress
- Pixel-style countdown digits in the side panel
- Focus mode and break mode
- Pause/resume during a session
- Clean quit with simple keyboard controls
- A completion state with a strong visual payoff

## Installation

Timepour is not published to crates.io yet.

For now, install from source:

```bash
cargo install --git https://github.com/Hyena0x/timepour timepour
```

Or clone and build locally:

```bash
git clone https://github.com/Hyena0x/timepour.git
cd timepour
cargo build --release
```

## Usage

Show help:

```bash
timepour --help
```

Start a default focus session (25 minutes):

```bash
timepour start
```

Start a custom focus session in minutes:

```bash
timepour start 15
```

Start a default break session (5 minutes):

```bash
timepour break
```

Start a custom break session in minutes:

```bash
timepour break 10
```

## Controls

- `p` — pause or resume
- `q` — quit
- `Esc` / `Enter` — exit the session

## Best used when

- You want a Pomodoro timer that feels alive
- You work mostly in the terminal
- You respond better to ambient visual pressure than passive clocks
- You want a lightweight ritual to begin focus sessions

## Design direction

Timepour is not trying to be a full productivity suite.

It is a single-purpose terminal object: open it, feel the session, finish the block, move on.

That constraint is the product.

## Roadmap ideas

- Sound toggle / optional completion audio themes
- Preset session profiles
- Better CLI ergonomics for seconds-based sessions
- Theme variants and display modes
- Optional packaged binaries and Homebrew distribution

## Contributing

Issues, ideas, UX feedback, and PRs are welcome.

- Bug reports: https://github.com/Hyena0x/timepour/issues
- Pull requests: https://github.com/Hyena0x/timepour/pulls

## License

MIT
