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

    /// Add an item to the TODO
    Add(AddArgs),
}

#[derive(Args, Debug)]
pub struct ShowArgs {
    #[arg(long)]
    pub all: bool,

    #[arg(long)]
    pub todo: bool,
}

#[derive(Args, Debug)]
pub struct AddArgs {
    /// Name of the TODO item
    #[arg(long)]
    pub name: String,

    /// Optional deadline date
    #[arg(long)]
    pub deadline: Option<String>,

    /// Optional item description
    #[arg(long)]
    pub desc: Option<String>,
}
