use clap::Parser;
use color_eyre::Result;
use timepour::{app, cli::Cli, timer::SessionKind};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    let kind = if cli.break_mode {
        SessionKind::Break
    } else {
        SessionKind::Focus
    };

    app::run(kind, cli.duration)
}
