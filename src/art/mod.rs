use crate::art::picture::Pic;
use raylib::color::Color;

pub mod picture;

pub struct Artist {
    pics: Vec<Pic>,
}

impl Artist {
    pub fn new() -> Self {
        Self { pics: Vec::new() }
    }

    pub fn reset(&mut self) {
        self.pics.clear();
    }

    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.pics.push(Pic::rect(x, y, w, h, color))
    }

    pub fn flush<F>(&mut self, callback: &mut F)
    where
        F: FnMut(Pic) -> (),
    {
        self.pics.sort_by(|a, b| {
            let bl = b.clone().layer();
            let al = a.clone().layer();
            al.total_cmp(&bl)
        });

        for pic in &self.pics {
            callback(pic.clone());
        }

        self.reset();
    }
}
