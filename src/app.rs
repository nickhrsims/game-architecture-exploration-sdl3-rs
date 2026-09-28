use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;

use std::process::exit;
use std::vec::Vec;

use crate::game;
use crate::grid::{Cell, Grid};
use crate::platform;

pub struct App {
    config: Config,
    state: AppState,
    platform: platform::Platform,
}

pub struct Config {
    pub name: &'static str,

    pub fps: u8,

    pub grid_width: u16,
    pub grid_height: u16,
    pub grid_scale: u8,
}

struct Session {
    game: game::Game,
    state: SessionState,
}

struct ExitStatus {
    code: i32,
    msg: &'static str,
}

struct HighResolutionCounter(u64, u64);

enum AppState {
    Initializing,
    MainMenu,
    Session(Session),
    Exiting(ExitStatus),
}

enum SessionState {
    Playing,
    Paused,
    Ending,
}

trait RenderTrait {
    fn draw_body(&mut self, scale: u8, body: &[Cell]);
    fn draw_head(&mut self, scale: u8, head: Cell);
    fn draw_apple(&mut self, scale: u8, apple: Cell);
    fn draw_grid(&mut self, scale: u8, grid: &Grid);
}

impl App {
    pub fn new(config: Config) -> Self {
        let state = AppState::Session(Session {
            game: game::Game::new(config.grid_width, config.grid_height),
            state: SessionState::Playing,
        });
        let platform = platform::Platform::init(platform::Config {
            window_title: config.name,
            window_width: config.grid_width as u32 * config.grid_scale as u32,
            window_height: config.grid_height as u32 * config.grid_scale as u32,
        });
        Self {
            config,
            state,
            platform,
        }
    }

    pub fn run(&mut self) {
        let ns_frame_cap = 1_000_000_000u32 / self.config.fps as u32;
        let mut ns_counter = HighResolutionCounter::new();

        loop {
            ns_counter.start();

            self.handle_platform_events();
            self.update();
            self.render();

            ns_counter.stop();

            ::std::thread::sleep(std::time::Duration::new(
                0,
                ns_frame_cap - ns_counter.nanoseconds(),
            ));
        }
    }

    fn handle_platform_events(&mut self) {
        for event in self.platform.events.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => self.state = AppState::Exiting(ExitStatus { code: 0, msg: "" }),
                _ => match self.state {
                    AppState::Initializing => {}
                    AppState::MainMenu => {}
                    AppState::Exiting(_) => {}
                    AppState::Session(ref mut session) => {
                        session.handle_platform_event(&event);
                    }
                },
            }
        }
    }

    fn update(&mut self) {
        match self.state {
            AppState::Initializing => {
                todo!();
            }
            AppState::MainMenu => {
                todo!()
            }
            AppState::Exiting(ref mut status) => {
                exit(status.code);
            }
            AppState::Session(ref mut session) => {
                session.update();
                if matches!(session.state, SessionState::Ending) {
                    self.state = AppState::Exiting(ExitStatus { code: 0, msg: "" })
                }
            }
        }
    }

    fn render(&mut self) {
        match self.state {
            AppState::Initializing => {}
            AppState::MainMenu => {}
            AppState::Exiting(_) => {}
            AppState::Session(ref mut session) => {
                session.render(&self.config, &mut self.platform.canvas);
            }
        }
    }
}

impl Session {
    pub fn handle_platform_event(&mut self, event: &sdl3::event::Event) {
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
            _ => {}
        }
    }

    pub fn update(&mut self) {
        // imm borrow happens here;
        // transition likely to become odd later.
        let state = &self.state;
        let game = &mut self.game;

        match state {
            SessionState::Playing => {
                game.step();
                if game.state == game::GameState::GameOver {
                    self.state = SessionState::Ending;
                }
            }
            SessionState::Paused => {}
            SessionState::Ending => {}
        }
    }

    pub fn render(&mut self, config: &Config, canvas: &mut Canvas<Window>) {
        let snapshot = self.game.snapshot();

        canvas.draw_grid(config.grid_scale, &self.game.grid);
        canvas.draw_head(config.grid_scale, snapshot.head);
        canvas.draw_body(config.grid_scale, snapshot.body);
        canvas.draw_apple(config.grid_scale, snapshot.apple);
        canvas.present();
    }
}

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

impl RenderTrait for Canvas<Window> {
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
