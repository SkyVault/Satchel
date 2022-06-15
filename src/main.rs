mod art;
mod data;
mod items;
mod userconf;

use crate::art::picture::PicKind;
use crate::art::Artist;
use crate::data::loader::load_items;
use crate::data::saver::save_items;
use crate::items::meta::Meta;
use crate::items::types::{Item, ItemKind};
use crate::userconf::UserConfig;
use raylib::consts::MouseButton;
use raylib::prelude::*;
use std::io;
use std::io::ErrorKind;

const GRID_SIZE: i32 = 32;

struct Input {
    mouse: Vector2,
    mouse_left_press: bool,
    mouse_left_release: bool,
}

struct Satchel {
    offset: Vector2,
    items: Vec<Item>,

    last_mx: f32,
    last_my: f32,
    dragging: bool,

    user_config: Option<UserConfig>,
}

impl Satchel {
    fn new() -> Self {
        Self {
            offset: Vector2::zero(),
            items: Vec::new(),
            last_mx: 0.0,
            last_my: 0.0,
            dragging: false,
            user_config: None,
        }
    }

    fn load(&mut self, config: UserConfig) {
        let mut loaded_items = match load_items(config.path.as_str()) {
            Ok(vs) => vs,
            Err(_err) => Vec::new(),
        };
        self.user_config = Some(config.clone());
        self.items.append(&mut loaded_items);
    }

    fn save(&self) {
        match &self.user_config {
            None => {}
            Some(config) => match save_items(config.path.as_str(), &self.items) {
                Err(err) => println!("Failed to save satchel {:?}", err),
                Ok(_) => println!("Saved!"),
            },
        }
    }
}

trait Update {
    fn update(&mut self, input: Input, dt: f64);
}
trait Init {
    fn init(&mut self);
}

impl Init for Satchel {
    fn init(&mut self) {
        // self.last_mx =
    }
}

impl Update for Satchel {
    fn update(&mut self, input: Input, dt: f64) {
        if self.dragging {
            self.offset.x += self.last_mx - input.mouse.x;
            self.offset.y += self.last_my - input.mouse.y;

            self.last_mx = input.mouse.x;
            self.last_my = input.mouse.y;
        }

        if input.mouse_left_press && !self.dragging {
            self.last_mx = input.mouse.x;
            self.last_my = input.mouse.y;
            self.dragging = true;
            return;
        }

        if input.mouse_left_release {
            self.dragging = false;
        }
    }
}

fn draw_grid(d: &mut RaylibMode2D<RaylibDrawHandle>) {
    for n in 0..(4000 / GRID_SIZE) + 1 {
        d.draw_line(
            -2000 + (n * GRID_SIZE),
            -2000,
            -2000 + (n * GRID_SIZE),
            2000,
            Color::DARKBLUE,
        );
        d.draw_line(
            -2000,
            -2000 + n * GRID_SIZE,
            2000,
            -2000 + n * GRID_SIZE,
            Color::DARKBLUE,
        );
    }
}

fn get_input(rl: &mut RaylibHandle) -> Input {
    let mut result = Input {
        mouse: Vector2::zero(),
        mouse_left_press: false,
        mouse_left_release: false,
    };

    result.mouse = Vector2::new(rl.get_mouse_x() as f32, rl.get_mouse_y() as f32);

    result.mouse_left_press = rl.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON);
    result.mouse_left_release = rl.is_mouse_button_released(MouseButton::MOUSE_LEFT_BUTTON);

    return result;
}

fn main() {
    let config = UserConfig::load().unwrap_or(UserConfig::default());
    config.save().unwrap();

    println!("Loaded user config {:?}", config);

    let mut satchel = Satchel::new();
    satchel.load(config);

    let mut artist = Artist::new();

    let mut camera = Camera2D {
        target: Vector2::new(0.0, 0.0),
        offset: Vector2::zero(),
        rotation: 0.0,
        zoom: 1.0,
    };

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Hello, World!")
        .build();

    while !rl.window_should_close() {
        let input = get_input(&mut rl);
        let dt = rl.get_frame_time() as f64;

        satchel.update(input, dt);

        camera.target = satchel.offset;

        let mut d = rl.begin_drawing(&thread);
        let mut d = d.begin_mode2D(camera);

        d.clear_background(Color::BLACK);

        artist.rect(10.0, 10.0, 100.0, 100.0, Color::GREEN);

        draw_grid(&mut d);
        d.draw_text("Hello!", 10, 10, 20, Color::YELLOW);

        artist.flush(&mut |p| match p.kind {
            PicKind::Rect => d.draw_rectangle(
                p.x as i32,
                p.y as i32,
                p.width as i32,
                p.height as i32,
                p.color,
            ),
            _ => {}
        });
    }
}
