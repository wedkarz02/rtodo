use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    #[arg(short, long)]
    asdf: String,
}

fn main() {
    let args = Cli::parse();
    println!("{:#?}", args);
}
