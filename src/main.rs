use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;

use std::vec::Vec;

pub mod game;
pub mod grid;

use game::{Game, GameState, Mode};
use grid::{Cell, Grid};

// -------------------------------------------------------------
// SDL3 Extensions
// -------------------------------------------------------------

struct HighResolutionCounter(u64, u64);

impl HighResolutionCounter {
    fn new() -> Self {
        Self(0, 0)
    }

    fn start(&mut self) {
        self.0 = sdl3::timer::performance_counter();
    }

    fn stop(&mut self) {
        self.1 = sdl3::timer::performance_counter();
    }

    fn nanoseconds(&self) -> u32 {
        ((1_000_000_000 * (self.1 - self.0)) / sdl3::timer::performance_frequency())
            .try_into()
            .expect("High resolution counter difference nanosecond-scaled fits into u32")
    }
}

trait RenderSnapshot {
    fn draw_body(&mut self, scale: u8, body: &[Cell]);
    fn draw_head(&mut self, scale: u8, head: Cell);
    fn draw_apple(&mut self, scale: u8, apple: Cell);
    fn draw_grid(&mut self, scale: u8, grid: &Grid);
}

impl RenderSnapshot for Canvas<Window> {
    fn draw_body(&mut self, scale: u8, body: &[Cell]) {
        let rects: Vec<FRect> = body
            .iter()
            .map(|cell| {
                FRect::new(
                    (cell.0 * scale as i32) as f32,
                    (cell.1 * scale as i32) as f32,
                    scale as f32,
                    scale as f32,
                )
            })
            .collect();
        self.set_draw_color(Color::RGB(80, 80, 80));
        self.fill_rects(&rects).expect("Rendering does not fail");
    }

    fn draw_head(&mut self, scale: u8, head: Cell) {
        let rect = FRect::new(
            head.0 as f32 * scale as f32,
            head.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );

        self.set_draw_color(Color::RGB(80, 80, 80));
        self.fill_rect(rect).expect("Rendering does not fail");
    }

    fn draw_apple(&mut self, scale: u8, apple: Cell) {
        let rect = FRect::new(
            apple.0 as f32 * scale as f32,
            apple.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );
        self.set_draw_color(Color::RGB(180, 80, 80));
        self.fill_rect(rect).expect("Rendering does not fail");
    }

    fn draw_grid(&mut self, scale: u8, grid: &Grid) {
        self.set_draw_color(Color::RGB(120, 120, 120));
        self.fill_rect(FRect::new(
            0f32,
            0f32,
            grid.width as f32 * scale as f32,
            grid.height as f32 * scale as f32,
        ))
        .unwrap();
    }
}

// -------------------------------------------------------------
// App Model
// -------------------------------------------------------------

// -------------------------------------------------------------
// Main
// -------------------------------------------------------------

fn main() {
    const TARGET_FPS: u8 = 10;
    const GRID_SCALE: u8 = 32;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake", GRID_SCALE as u32 * 24, GRID_SCALE as u32 * 24)
        .resizable()
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas();

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
    canvas.present();

    let ns_frame_cap = 1_000_000_000u32 / TARGET_FPS as u32;
    let mut ns_counter = HighResolutionCounter::new();
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new(Mode::Normal);

    while game.state == GameState::Playing {
        ns_counter.start();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => game.state = GameState::GameOver,
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => game.north(),
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => game.south(),
                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => game.west(),
                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => game.east(),
                _ => {}
            }
        }

        game.step();

        let snapshot = game.snapshot();
        canvas.draw_grid(GRID_SCALE, &game.grid);
        canvas.draw_head(GRID_SCALE, snapshot.head);
        canvas.draw_body(GRID_SCALE, snapshot.body);
        canvas.draw_apple(GRID_SCALE, snapshot.apple);
        canvas.present();

        ns_counter.stop();

        ::std::thread::sleep(std::time::Duration::new(
            0,
            ns_frame_cap - ns_counter.nanoseconds(),
        ));
    }
}
