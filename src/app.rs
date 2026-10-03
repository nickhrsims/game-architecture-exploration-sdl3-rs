// -------------------------------------------------------------
// Modules
// -------------------------------------------------------------

mod block;
mod event;
mod icon;
mod rect;
mod video;

// -------------------------------------------------------------
// Imports
// -------------------------------------------------------------

// -- SDL3

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;

// -- Standard Library

use std::process::exit;

// -- Crate

use crate::game;
use crate::geom::{Grid, Point, Size, Transform};

// -- Local

use block::Block;

// -------------------------------------------------------------
// Type Aliases
// -------------------------------------------------------------

type Error = Box<dyn std::error::Error>;

// -------------------------------------------------------------
// Application
// -------------------------------------------------------------

/// Application API root.
pub struct App {
    state: AppState,
    config: Config,
    #[allow(unused)]
    sdl: sdl3::Sdl,
    #[allow(unused)]
    ttf: sdl3::ttf::Sdl3TtfContext,
    video: video::Video,
    events: event::EventPump,
}

// -----------------------------
// App :: Public API
// -----------------------------

impl App {
    pub fn init(config: Config) -> Result<Self, Error> {
        let state = AppState::InGame(Gameplay {
            game: game::Game::new(Grid::new(Size::new(config.grid_width, config.grid_height))),
            state: GameplayState::Playing,
        });

        let sdl = sdl3::init()?;
        let ttf = sdl3::ttf::init()?;
        let video = video::configure()
            .window(|w| w.title("Snake").size((640, 480)))
            .rendering(|r| r.alpha_blending())
            .build(&sdl)?;

        let events = event::EventPump::init(&sdl)?;

        Ok(Self {
            // Application + Game Boundary
            state,

            // Application + World Boundary
            config,

            // Application + Platform Boundary
            sdl,
            ttf,
            video,
            events,
        })
    }

    pub fn run(&mut self) {
        let ns_frame_cap = 1_000_000_000u32 / self.config.fps as u32;
        let mut ns_counter = HighResolutionCounter::new();

        loop {
            ns_counter.start();

            self.handle_platform_events();
            self.update();

            if let Err(error) = self.render() {
                panic!("Encountered SDL3 Rendering error: {error}");
            }

            ns_counter.stop();

            ::std::thread::sleep(std::time::Duration::new(
                0,
                ns_frame_cap - ns_counter.nanoseconds(),
            ));
        }
    }
}

// -----------------------------
// App :: Private Impl
// -----------------------------

impl App {
    fn handle_platform_events(&mut self) {
        for event in self.events.iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => {
                    self.state = AppState::Exiting(ExitStatus {
                        code: 0,
                        msg: "\nBye!",
                    })
                }
                _ => match self.state {
                    AppState::Exiting(_) => {}
                    AppState::InGame(ref mut session) => {
                        session.handle_platform_event(&event);
                    }
                },
            }
        }
    }

    fn update(&mut self) {
        match self.state {
            AppState::Exiting(ref mut status) => {
                let msg = status.msg;
                eprintln!("{msg}");
                exit(status.code);
            }
            AppState::InGame(ref mut gameplay) => {
                gameplay.update();
                if gameplay.state == GameplayState::Ending {
                    self.state = AppState::Exiting(ExitStatus {
                        code: 0,
                        msg: "\nYou Died!",
                    })
                }
            }
        }
    }

    fn render(&mut self) -> Result<(), Error> {
        self.video.canvas.set_draw_color(Color::BLACK);
        self.video.canvas.clear();
        match self.state {
            AppState::Exiting(_) => {}
            AppState::InGame(ref mut session) => {
                session.render(&self.config, &mut self.video)?;
            }
        }
        self.video.canvas.present();
        Ok(())
    }
}

/// Top-level Application configuration.
pub struct Config {
    pub name: &'static str,

    pub fps: u8,

    pub grid_width: u32,
    pub grid_height: u32,
    pub grid_scale: u8,
}

// -------------------------------------------------------------
// Application FSM
// -------------------------------------------------------------

// -----------------------------
// Root FSM
// -----------------------------

enum AppState {
    InGame(Gameplay),
    Exiting(ExitStatus),
}

// -----------------------------
// InGame
// -----------------------------

struct Gameplay {
    game: game::Game,
    state: GameplayState,
}

#[derive(PartialEq)]
enum GameplayState {
    Playing,
    Paused { icon: icon::Pause },
    Ending,
}

impl Gameplay {
    fn handle_platform_event(&mut self, event: &sdl3::event::Event) {
        match self.state {
            GameplayState::Playing => {
                let game = &mut self.game;
                match event {
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
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => {
                        self.state = GameplayState::Paused {
                            icon: icon::Pause::new(),
                        }
                    }

                    _ => {}
                }
            }

            GameplayState::Paused { .. } => {
                if let Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } = event
                {
                    self.state = GameplayState::Playing;
                } else {
                    // TODO:
                    // pass it along to the pause menu
                }
            }

            GameplayState::Ending => {
                // no-op
            }
        }
    }

    fn update(&mut self) {
        // imm borrow happens here;
        // transition likely to become odd later.
        let state = &self.state;
        let game = &mut self.game;

        match state {
            GameplayState::Playing => {
                game.step();
                if game.state == game::State::GameOver {
                    self.state = GameplayState::Ending;
                }
            }
            GameplayState::Paused { .. } => {}
            GameplayState::Ending => {}
        }
    }

    fn render(&mut self, config: &Config, video: &mut video::Video) -> Result<(), Error> {
        let snapshot = self.game.snapshot();

        video.canvas.draw_grid(config.grid_scale, &self.game.grid)?;
        video.canvas.draw_head(config.grid_scale, snapshot.head)?;
        video.canvas.draw_body(config.grid_scale, snapshot.body)?;
        video.canvas.draw_apple(config.grid_scale, snapshot.apple)?;

        if let GameplayState::Paused { icon } = &self.state {
            let grid_rect = FRect::new(
                0f32,
                0f32,
                config.grid_width as f32 * config.grid_scale as f32,
                config.grid_height as f32 * config.grid_scale as f32,
            );

            video.canvas.set_draw_color(Color::RGBA(0, 0, 0, 120));
            video.canvas.fill_rect(grid_rect)?;

            let window_width = (config.grid_width * config.grid_scale as u32) as i32;
            let window_height = (config.grid_height * config.grid_scale as u32) as i32;

            let grid_transform = Transform::from(Point(
                window_width - (config.grid_scale as i32 * 2),
                window_height - (config.grid_scale as i32 * 2),
            ));

            icon.blocks()
                .map(|block| block.transform(grid_transform))
                .try_for_each(|block| video.draw_block(&block))?;
        }

        Ok(())
    }
}

// -----------------------------
// Exiting
// -----------------------------

struct ExitStatus {
    code: i32,
    msg: &'static str,
}

// -------------------------------------------------------------
// Supporting Machinery
// -------------------------------------------------------------

// -----------------------------
// Timing
// -----------------------------

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

// -----------------------------
// Rendering
// -----------------------------

trait AppCanvas {
    fn draw_body(&mut self, scale: u8, body: &[Point]) -> Result<(), sdl3::Error>;
    fn draw_head(&mut self, scale: u8, head: Point) -> Result<(), sdl3::Error>;
    fn draw_apple(&mut self, scale: u8, apple: Point) -> Result<(), sdl3::Error>;
    fn draw_grid(&mut self, scale: u8, grid: &Grid) -> Result<(), sdl3::Error>;
}

impl AppCanvas for Canvas<Window> {
    fn draw_body(&mut self, scale: u8, body: &[Point]) -> Result<(), sdl3::Error> {
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
        self.fill_rects(&rects)?;
        Ok(())
    }

    fn draw_head(&mut self, scale: u8, head: Point) -> Result<(), sdl3::Error> {
        let rect = FRect::new(
            head.0 as f32 * scale as f32,
            head.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );

        self.set_draw_color(Color::RGB(80, 80, 80));
        self.fill_rect(rect)?;
        Ok(())
    }

    fn draw_apple(&mut self, scale: u8, apple: Point) -> Result<(), sdl3::Error> {
        let rect = FRect::new(
            apple.0 as f32 * scale as f32,
            apple.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );
        self.set_draw_color(Color::RGB(180, 80, 80));
        self.fill_rect(rect)?;
        Ok(())
    }

    fn draw_grid(&mut self, scale: u8, grid: &Grid) -> Result<(), sdl3::Error> {
        self.set_draw_color(Color::RGB(120, 120, 120));
        self.fill_rect(FRect::new(
            0f32,
            0f32,
            grid.width() as f32 * scale as f32,
            grid.height() as f32 * scale as f32,
        ))?;
        Ok(())
    }
}
