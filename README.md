<div align="center">
  <h1>🎮 Timepour</h1>
  <p><strong>A visceral, block-stacking terminal countdown with deterministic Tetris-style physics.</strong></p>

  [![Crates.io](https://img.shields.io/crates/v/timepour.svg)](https://crates.io/crates/timepour)
  [![License](https://img.shields.io/crates/l/timepour.svg)](https://github.com/Hyena0x/timepour/blob/main/LICENSE)
  [![Build Status](https://img.shields.io/github/actions/workflow/status/Hyena0x/timepour/rust.yml)](https://github.com/Hyena0x/timepour/actions)
</div>

<br />

**Timepour** transforms your boring productivity sessions into an immersive arcade experience. Instead of watching a standard clock tick down, watch a deterministic, pro-level AI perfectly pack a Tetris matrix as your time passes.

When your time is up, the vibrant blocks wash away into a dead monochrome silence.

## ✨ Features

- **Pro-Player Packing AI**: A custom heuristic algorithm simulates perfect 4D rotations and horizontal glides, packing the board tightly without gaps.
- **Time Synchronization**: The stack perfectly syncs to your timeline. When the screen fills, your time is exactly up.
- **Immersive Arcade UI**: Featuring an integrated 5x5 pixel-art clock that visually syncs to the session color schema.
- **Contextual Ambient Lighting**: Red UI boardings for intense *Focus* sessions, and mint-cyan for *Break* sessions.
- **Grayscale Death Effect**: The entire board grays out dynamically at the exact millisecond the countdown reaches zero.

## 🚀 Installation

Ensure you have Rust installed, then run:

```bash
cargo install timepour
```

*(Alternatively, clone this repository and run `cargo build --release`)*

## 🕹️ Usage

Timepour uses a dead-simple declarative CLI interface contextually modeled around standard Pomodoro structures.

### Focus Mode (Default 25 minutes)
```bash
timepour start
```

### Custom Focus Duration (e.g., 1 hour and 30 seconds)
```bash
timepour start 60 30
```

### Break Mode
```bash
timepour break 5 0
```

### In-App Controls
- `p` - Pause / Resume the timer and the falling blocks.
- `q` - Quit immediately.

## 🧠 Why we built this

Traditional desktop productivity timers can sit quietly to the point of being ignored, or become an anxiety-inducing numeric spinner. Timepour was designed to offload time anxiety into an autonomous aesthetic artifact. 

## 🤝 Contributing

Contributions, issues, and feature requests are welcome! 
Feel free to check [issues page](https://github.com/Hyena0x/timepour/issues).

## 📝 License

This project is [MIT](https://github.com/yourusername/timepour/blob/master/LICENSE) licensed.
