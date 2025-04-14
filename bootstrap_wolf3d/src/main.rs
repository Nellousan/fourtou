use sfml::{
    graphics::{Color, Image, Rect, RenderTarget, RenderWindow, Sprite, Texture},
    system::{Vector2, Vector2f},
    window::{ContextSettings, Scancode, Style, VideoMode},
};

#[derive(Debug, Default)]
pub struct Player {
    pub pos: Vector2f,
    pub angle: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            pos: Vector2f::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            angle: 0f32,
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
    pub fn is_wall(&self, pos: Vector2<usize>) -> Option<&bool> {
        self.inner.get(pos.y * MAP_WIDTH + pos.x)
    }

    pub fn set_tile(&mut self, pos: Vector2<usize>, wall: bool) {
        self.inner[pos.y * MAP_WIDTH + pos.x] = wall;
    }

    pub fn add_some_walls(&mut self) {
        self.set_tile(Vector2::new(2, 1), true);
        self.set_tile(Vector2::new(2, 2), true);
        self.set_tile(Vector2::new(2, 3), true);
        self.set_tile(Vector2::new(2, 4), true);
        self.set_tile(Vector2::new(2, 5), true);
        self.set_tile(Vector2::new(2, 6), true);
    }
}

const TILE_SIZE: f32 = 64.0;
const MAP_WIDTH: usize = 8;
#[expect(dead_code)]
const MAP_HEIGHT: usize = 8;
const WINDOW_HEIGHT: u32 = 600;
const WINDOW_WIDTH: u32 = 800;
const FOV: f32 = 60.0;
const RENDER_DISTANCE: f32 = 600.0;
const MARCHER_DIRECTION_COEFFICIENT: f32 = TILE_SIZE / 4.0;

fn main() -> eyre::Result<()> {
    let mut map = Map::default();
    let mut player = Player::new();
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

            if let sfml::window::Event::KeyPressed {
                code: _,
                scan,
                alt: _,
                ctrl: _,
                shift: _,
                system: _,
            } = event
            {
                if scan == Scancode::D {
                    player.angle += 10.0;
                    player.angle %= 360.0;
                }
                if scan == Scancode::A {
                    player.angle -= 10.0;
                    player.angle %= 360.0;
                }
            }
        }

        window.clear(Color::rgb(0, 0, 0));

        draw_floor_and_ceiling(&mut window)?;
        cast_all_rays(&mut window, &player, &map)?;

        window.display();
    }

    Ok(())
}

fn to_map_coord(pos: Vector2f) -> Vector2<usize> {
    let res_x = (pos.x / TILE_SIZE).floor();
    let res_y = (pos.y / TILE_SIZE).floor();

    Vector2::new(res_x as usize, res_y as usize)
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

fn cast_all_rays(window: &mut RenderWindow, player: &Player, map: &Map) -> eyre::Result<()> {
    let mut pixel_buffer = Image::new_solid(WINDOW_WIDTH, WINDOW_HEIGHT, Color::rgba(0, 0, 0, 0))?;

    for i in 1..=WINDOW_WIDTH {
        let effective_angle = player.angle + (FOV * (i as f32 / WINDOW_WIDTH as f32));

        let Some(distance) = cast_single_ray(player, map, effective_angle.to_radians()) else {
            continue;
        };

        // let corrected_dist =
        //     (distance * f32::cos(effective_angle - player.angle).to_radians()).abs();
        // let wall_height = TILE_SIZE / corrected_dist;
        let wall_height = TILE_SIZE * (RENDER_DISTANCE / distance);

        let lower_bound = (WINDOW_HEIGHT as i32 / 2) - wall_height as i32 / 2;
        let upper_bound = (WINDOW_HEIGHT as i32 / 2) + wall_height as i32 / 2;

        // dbg!(
        //     effective_angle,
        //     distance,
        //     wall_height,
        //     lower_bound,
        //     upper_bound
        // );

        for j in 0..WINDOW_HEIGHT {
            if (j as i32) > lower_bound && (j as i32) < upper_bound {
                pixel_buffer.set_pixel(i - 1, j, Color::rgb(200, 100, 100))?
            }
        }
        let texture = Texture::from_image(
            &pixel_buffer,
            Rect::new(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32),
        )?;
        let sprite = Sprite::with_texture(&texture);

        window.draw(&sprite);
    }

    Ok(())
}

fn vector_distance(lhs: &Vector2f, rhs: &Vector2f) -> f32 {
    let x_side = (rhs.x - lhs.x).powi(2);
    let y_side = (rhs.y - lhs.y).powi(2);

    (x_side + y_side).sqrt()
}

fn cast_single_ray(player: &Player, map: &Map, ray_angle: f32) -> Option<f32> {
    let direction = Vector2f::new(f32::sin(ray_angle), f32::cos(ray_angle));
    let mut marcher = player.pos.clone();

    while vector_distance(&player.pos, &marcher) < RENDER_DISTANCE {
        marcher += direction * MARCHER_DIRECTION_COEFFICIENT;

        if let Some(true) = map.is_wall(to_map_coord(marcher)) {
            dbg!(marcher, to_map_coord(marcher));
            return Some(vector_distance(&player.pos, &marcher));
        }
    }

    None
}
