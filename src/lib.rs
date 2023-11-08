use dirs;
use std::{fs, io, path::Path};

#[derive(Debug)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub desc: String,
}

impl Item {
    pub fn new(id: u32, name: String, desc: Option<String>) -> Self {
        let desc = if let Some(d) = desc {
            d
        } else {
            String::from("-")
        };

        return Self { id, name, desc };
    }
}

pub fn init_all() -> Result<(), io::Error> {
    let mut dir_path = dirs::home_dir().expect("Home directory should exist");
    dir_path.push(".rtodo");

    if let Ok(_) = fs::create_dir(&dir_path) {
        println!("[INFO]: ~/.rtodo directory created succesfully.");
    } else {
        println!("[INFO]: ~/.rtodo already exists. Clearing data...");
    }

    dir_path.push("items.txt");
    let file_path = Path::new(&dir_path);

    fs::write(file_path, "")?;
    println!("[INFO]: ~/.rtodo/items.txt created succesfully.");

    Ok(())
}
