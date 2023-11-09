use std::{
    fs::{self, File, OpenOptions},
    io::{self, prelude::*, BufRead, BufReader},
    path::Path,
};

#[derive(Debug)]
pub struct Item {
    pub id: usize,
    pub name: String,
    pub deadline: String,
    pub desc: String,
}

impl Item {
    pub fn new(id: usize, name: String, deadline: String, desc: String) -> Self {
        return Self {
            id,
            name,
            deadline,
            desc,
        };
    }

    pub fn as_string(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}",
            self.id, self.name, self.deadline, self.desc
        )
    }
}

pub fn init_all() -> Result<(), io::Error> {
    let mut home_path = dirs::home_dir().expect("Home directory should exist");
    home_path.push(".rtodo");

    if let Ok(_) = fs::create_dir(&home_path) {
        println!("[INFO]: ~/.rtodo directory created succesfully.");
    } else {
        println!("[INFO]: ~/.rtodo already exists. Clearing data...");
    }

    home_path.push("items.txt");
    let file_path = Path::new(&home_path);

    fs::write(file_path, "")?;
    println!("[INFO]: ~/.rtodo/items.txt created succesfully.");

    Ok(())
}

pub fn count_lines(file_name: &Path) -> Result<usize, io::Error> {
    let file = File::open(file_name)?;
    let buffered = BufReader::new(file);

    Ok(buffered.lines().count())
}

pub fn write_item(file_path: &Path, item: Item) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&file_path)
        .unwrap();

    if let Err(e) = writeln!(file, "{}", item.as_string()) {
        eprintln!("[ERROR]: {}", e);
    }
}
