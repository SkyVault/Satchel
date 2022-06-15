use serde::{Deserialize, Serialize};
use serde_json::Error as JsError;
use std::fs;
use std::fs::{read_to_string, File};
use std::io::{Error, Result, Write};

const CONFIG_PATH: &str = "/home/dustin/.config/satchel.json";
const DEFAULT_PATH: &str = "/home/dustin/.config/bookmarks.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserConfig {
    pub path: String,
}

impl UserConfig {
    pub fn save(&self) -> Result<()> {
        let serialized = serde_json::to_string(self).unwrap();
        let mut file = File::create(CONFIG_PATH)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    pub fn load() -> Result<UserConfig> {
        let contents = read_to_string(CONFIG_PATH)?;
        let unser: UserConfig = serde_json::from_str(contents.as_str()).unwrap();
        Ok(unser)
    }

    pub fn default() -> UserConfig {
        UserConfig {
            path: String::from(DEFAULT_PATH),
        }
    }
}
