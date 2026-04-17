<div align="center">
  <h1>🎮 Timepour</h1>
  <p><strong>A terminal focus timer that turns passing time into a falling tetromino stack.</strong></p>

  [![License](https://img.shields.io/github/license/Hyena0x/timepour)](https://github.com/Hyena0x/timepour/blob/main/LICENSE)
  [![CI](https://img.shields.io/github/actions/workflow/status/Hyena0x/timepour/rust.yml?label=ci)](https://github.com/Hyena0x/timepour/actions)
</div>

Timepour makes a focus session feel physical.

Instead of watching a clock tick down, you watch the terminal slowly fill with a deterministic stack of falling blocks. Focus runs glow red. Breaks cool down. When time is up, the board drops into grayscale and the session is over.

Small tool. Strong presence.

## Why Timepour

- More vivid than a silent countdown
- Terminal-native and instant to launch
- Deterministic visual progression, not random noise
- Built for short focus rituals, not bloated productivity systems

## Install

Not on crates.io yet.

```bash
cargo install --git https://github.com/Hyena0x/timepour timepour
```

Or build locally:

```bash
git clone https://github.com/Hyena0x/timepour.git
cd timepour
cargo build --release
```

## Usage

```bash
# 25-minute focus session
timepour start

# 15-minute focus session
timepour start 15

# 5-minute break
timepour break

# 10-minute break
timepour break 10
```

## Controls

- `p` pause / resume
- `q` quit
- `Esc` / `Enter` exit session

## Contributing

Ideas, issues, and PRs are welcome.

## License

MIT
