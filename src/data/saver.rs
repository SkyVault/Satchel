use crate::Item;
use std::fs::File;
use std::io;
use std::io::{Error, ErrorKind, Write};

fn serialize_items(items: &Vec<Item>) -> Result<String, String> {
    let serialized = serde_json::to_string(items);

    match serialized {
        Ok(s) => Ok(s),
        Err(e) => Err(String::from("")),
    }
}

pub fn save_items(path: &str, items: &Vec<Item>) -> io::Result<()> {
    let ser = serialize_items(items);

    match ser {
        Ok(contents) => {
            let mut file = File::create(path)?;
            file.write_all(contents.as_bytes())?;
            Ok(())
        }
        Err(msg) => Err(Error::new(ErrorKind::Other, msg)),
    }
}
