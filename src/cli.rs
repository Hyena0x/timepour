use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "timepour",
    version,
    about = "An arcade-style terminal focus timer with a deterministic tetromino stack"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Commands {
    Start {
        minutes: Option<u64>,
        seconds: Option<u64>,
    },
    Break {
        minutes: Option<u64>,
    },
}
