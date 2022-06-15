use crate::items::meta::Meta;
use raylib::math::Vector2;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ItemKind {
    UrlBookmark(String),
    FileBookmark(String),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub(crate) position: (f32, f32),
    pub(crate) kind: ItemKind,
    pub(crate) meta: Meta,
}
