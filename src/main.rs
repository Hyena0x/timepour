use clap::Parser;
use color_eyre::Result;
use timepour::{
    app,
    cli::{Cli, Commands},
    timer::SessionKind,
};

fn main() -> Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();

    match cli.command {
        Commands::Start { minutes, seconds } => app::run(SessionKind::Focus, minutes, seconds),
        Commands::Break { minutes } => app::run(SessionKind::Break, minutes, None),
    }
}
