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

Install from crates.io:

```bash
cargo install timepour --locked
```

Or install a specific GitHub release tag:

```bash
cargo install --git https://github.com/Hyena0x/timepour.git --tag v0.1.2 --locked
```

Build locally:

```bash
git clone https://github.com/Hyena0x/timepour.git
cd timepour
cargo build --release --locked
```

## Usage

Durations are in minutes by default. You can also use `15m`, `90s`, `1:30`, `1h30m`, or `1m30s`.

```bash
# 25-minute focus session
timepour 25

# 25-minute focus session, explicit minutes
timepour 25m

# 90-minute focus session
timepour 1h30m

# 90-second focus session
timepour 90s

# 5-minute break
timepour --break 5

# 5-minute break, short flag
timepour -b 5m
```

## Controls

- `p` pause / resume
- `q` quit
- `Esc` / `Enter` exit session

## Contributing

Ideas, issues, and PRs are welcome.

## License

MIT
