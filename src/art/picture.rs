use raylib::color::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub enum PicKind {
    Rect,
    Text(String),
}

#[derive(Clone, Debug)]
pub struct Pic {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub color: Color,
    pub layer: f32,
    pub kind: PicKind,
}

impl Pic {
    pub fn new() -> Self {
        Pic {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            color: Color::WHITE,
            layer: 1.0,
            kind: PicKind::Rect,
        }
    }

    pub fn layer(self, new_layer: f32) -> Pic {
        Pic {
            layer: new_layer,
            ..self
        }
    }

    pub fn kind(self, new_kind: PicKind) -> Pic {
        Pic {
            kind: new_kind,
            ..self
        }
    }

    pub fn rect(x: f32, y: f32, width: f32, height: f32, color: Color) -> Pic {
        Pic {
            x,
            y,
            width,
            height,
            color,
            layer: 1.0,
            kind: PicKind::Rect,
        }
    }
}
