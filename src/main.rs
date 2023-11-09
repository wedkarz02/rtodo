use std::{fs, path::Path};

use clap::Parser;
use rtodo;

mod cli;

fn main() {
    let args = cli::Cli::parse();

    let mut home_path = dirs::home_dir().unwrap_or_else(|| {
        eprintln!("Home directory should exist");
        std::process::exit(1);
    });

    home_path.push(".rtodo/items.txt");
    let file_path = Path::new(&home_path);

    match &args.command {
        cli::Commands::Show(arg) => match (arg.all, arg.todo) {
            (true, false) => {
                let contents = fs::read_to_string(&file_path);

                match contents {
                    Ok(c) => println!("{}", c),
                    Err(e) => {
                        eprintln!("[ERROR]: {}", e);
                        std::process::exit(1);
                    }
                };
            }
            (false, true) => println!("todo"),
            _ => {}
        },
        cli::Commands::Init => {
            if let Err(e) = rtodo::init_all() {
                eprintln!("[ERROR]: {}", e);
            }
        }
        cli::Commands::Add(arg) => {
            let name = &arg.name;
            let deadline = if let Some(dl) = &arg.deadline {
                dl
            } else {
                "-"
            };
            let desc = if let Some(d) = &arg.desc { d } else { "-" };

            let id = match rtodo::count_lines(file_path) {
                Ok(i) => i,
                Err(e) => {
                    eprintln!("[ERROR]: {}", e);
                    std::process::exit(1)
                }
            };

            let item = rtodo::Item::new(
                id,
                String::from(name),
                String::from(deadline),
                String::from(desc),
            );
            rtodo::write_item(&file_path, item);
        }
    }
}
