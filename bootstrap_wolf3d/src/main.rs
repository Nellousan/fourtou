use std::f32::consts::PI;

use sfml::{
    graphics::{Color, Image, Rect, RenderTarget, RenderWindow, Sprite, Texture},
    window::{ContextSettings, Style, VideoMode},
};

#[derive(Debug, Default)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: TILE_SIZE / 2.0,
            y: TILE_SIZE / 2.0,
            angle: 90f32,
        }
    }
}

#[derive(Debug)]
pub struct Map {
    pub inner: [bool; 64],
}

impl Default for Map {
    fn default() -> Self {
        Self { inner: [false; 64] }
    }
}

impl Map {
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.inner[y * MAP_WIDTH + x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, wall: bool) {
        self.inner[y * MAP_WIDTH + x] = wall;
    }

    pub fn get_tile(&self, x: usize, y: usize) -> bool {
        self.inner[y * MAP_WIDTH + x]
    }

    pub fn add_some_walls(&mut self) {
        self.set_tile(0, 0, true);
        self.set_tile(0, 1, true);
        self.set_tile(0, 2, true);
        self.set_tile(0, 3, true);
        self.set_tile(0, 4, true);
        self.set_tile(0, 5, true);
        self.set_tile(0, 6, true);
    }
}

const TILE_SIZE: f32 = 64.0;
const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const FOV: f32 = 60.0;

fn main() -> eyre::Result<()> {
    let mut map = Map::default();
    map.add_some_walls();

    let context_settings = ContextSettings::default();

    let mut window = RenderWindow::new(
        VideoMode::new(WINDOW_WIDTH, WINDOW_HEIGHT, 32),
        "yuh",
        Style::RESIZE,
        &context_settings,
    )?;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            if event == sfml::window::Event::Closed {
                window.close();
            }
        }

        window.clear(sfml::graphics::Color::rgb(0, 0, 0));

        draw_floor_and_ceiling(&mut window)?;

        window.display();
    }

    Ok(())
}

fn to_map_coord(x: f32, y: f32) -> (usize, usize) {
    let res_x = x / TILE_SIZE;
    let res_y = y / TILE_SIZE;

    (res_x as usize, res_y as usize)
}

fn draw_floor_and_ceiling(window: &mut RenderWindow) -> eyre::Result<()> {
    let mut pixel_buffer =
        Image::new_solid(WINDOW_WIDTH, WINDOW_HEIGHT, Color::rgb(135, 206, 235))?;

    for i in 0..(WINDOW_HEIGHT / 2) {
        for j in 0..WINDOW_WIDTH {
            pixel_buffer.set_pixel(j, i + WINDOW_HEIGHT / 2, Color::rgb(60, 60, 60))?;
        }
    }

    let texture = Texture::from_image(
        &pixel_buffer,
        Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32),
    )?;
    let sprite = Sprite::with_texture(&texture);

    window.draw(&sprite);

    Ok(())
}

fn cast_all_rays(window: &mut RenderWindow, player: &Player) -> eyre::Result<()> {
    for i in 1..=WINDOW_HEIGHT {
        let effective_angle = player.angle + (FOV * (i as f32 / WINDOW_HEIGHT as f32));
    }

    Ok(())
}

fn cast_single_ray(player: &Player, ray_angle: f32) -> f32 {
    unimplemented!()
}
