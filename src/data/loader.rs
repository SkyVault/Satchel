use crate::Item;
use std::fs::read_to_string;
use std::io::Result;

fn deserialize_items(path: &str) -> Result<Vec<Item>> {
    let contents = read_to_string(path)?;
    let result: Vec<Item> = serde_json::from_str(contents.as_str()).unwrap_or(Vec::new());
    Ok(result)
}

pub fn load_items(path: &str) -> Result<Vec<Item>> {
    deserialize_items(path)
}
