use clap::Parser;
// use rtodo;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    println!("{:#?}", args);

    let mut db: Vec<String> = vec![];
    db.push(String::from("hehe"));
    db.push(String::from("asdf"));
    db.push(String::from("fdsa"));

    match &args.command {
        cli::Commands::Show(arg) => match (arg.all, arg.todo) {
            (true, false) => {
                for row in db {
                    println!("{:#?}", row);
                }
            }
            (false, true) => println!("todo"),
            _ => {}
        },
    }
}
