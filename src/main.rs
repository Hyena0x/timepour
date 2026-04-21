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
        Commands::Start { duration } => app::run(SessionKind::Focus, duration),
        Commands::Break { duration } => app::run(SessionKind::Break, duration),
    }
}
