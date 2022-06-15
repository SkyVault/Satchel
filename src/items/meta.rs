use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    title: String,
}

impl Meta {
    pub fn new(title: &str) -> Self {
        Meta {
            title: String::from(title),
        }
    }
}
