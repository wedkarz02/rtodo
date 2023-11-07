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

// pub fn save_item(item: &Item, file_path: &str) -> Result<()>
