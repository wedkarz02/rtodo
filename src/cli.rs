use clap::{Args, Parser, Subcommand};

/// rtodo - Rusty TODO
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize / clear application files
    Init,

    /// Display TODO items
    Show(ShowArgs),
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub todo: bool,
}
