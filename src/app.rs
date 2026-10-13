// -----------------------------------------------------------------------------
//                                                                       Imports
// -----------------------------------------------------------------------------

// -- SDL3

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;

// -- Standard Library

use std::process::exit;

// -- Crate

use crate::game;

use crate::geom::{Grid, Point, Rect, Size};
use crate::platform::Platform;
use crate::platform::text::FontId;

use crate::display::{Block, DisplayCommands, Text};

// -----------------------------------------------------------------------------
//                                                                  Type Aliases
// -----------------------------------------------------------------------------

type AnyError = Box<dyn std::error::Error>;

// -----------------------------------------------------------------------------
//                                                                     Constants
// -----------------------------------------------------------------------------

const FONT_CALIPER_ID: FontId = FontId("caliper");
const FONT_CALIPER_PATH: &str = "./data/font/caliper.ttf";

// -----------------------------------------------------------------------------
//                                                         Application :: Public
// -----------------------------------------------------------------------------

/// Application API root.
pub struct App {
    state: Mode,
    config: Config,
    platform: Platform,
}

impl App {
    pub fn init(config: Config) -> Result<Self, AnyError> {
        let state = Mode::InGame(Gameplay {
            game: game::Game::new(Grid::new(Size::new(config.grid_width, config.grid_height))),
            state: GameplayState::Playing,
        });

        let mut platform = Platform::init()?;

        platform.register_font(FONT_CALIPER_ID, FONT_CALIPER_PATH);

        Ok(Self {
            state,
            config,
            platform,
        })
    }

    pub fn run(&mut self) {
        let ns_frame_cap = 1_000_000_000u32 / self.config.fps as u32;
        let mut timer = self.platform.timer();

        loop {
            timer.start();

            self.handle_events();
            self.update();
            self.draw().expect("Drawing does not fail");

            timer.stop();

            ::std::thread::sleep(std::time::Duration::new(
                0,
                ns_frame_cap - timer.nanoseconds(),
            ));
        }
    }
}

pub struct Config {
    pub name: &'static str,

    pub fps: u8,

    pub grid_width: u32,
    pub grid_height: u32,
    pub grid_scale: u8,
}

// -----------------------------------------------------------------------------
//                                            Application :: Processing Pipeline
// -----------------------------------------------------------------------------

impl App {
    fn handle_events(&mut self) {
        for event in self.platform.events() {
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
                    self.state = Mode::Exiting(ExitStatus {
                        code: 0,
                        msg: "\nBye!",
                    })
                }
                _ => match self.state {
                    Mode::Exiting(_) => {}
                    Mode::InGame(ref mut session) => {
                        session.handle_platform_event(&event);
                    }
                },
            }
        }
    }

    fn update(&mut self) {
        match self.state {
            Mode::Exiting(ref mut status) => {
                let msg = status.msg;
                eprintln!("{msg}");
                exit(status.code);
            }
            Mode::InGame(ref mut gameplay) => {
                gameplay.update();
                if gameplay.state == GameplayState::Ending {
                    self.state = Mode::Exiting(ExitStatus {
                        code: 0,
                        msg: "\nYou Died!",
                    })
                }
            }
        }
    }

    fn draw(&mut self) -> Result<(), AnyError> {
        let mut frame = DisplayCommands::new(
            &mut self.platform.display_system,
            &mut self.platform.text_system,
        );
        frame.clear();
        match self.state {
            Mode::Exiting(_) => {}
            Mode::InGame(ref mut session) => {
                session.draw(&mut frame, self.config.grid_scale)?;
            }
        }
        frame.present()?;
        Ok(())
    }
}

// -------------------------------------------------------------
// Application FSM
// -------------------------------------------------------------

// -----------------------------
// Root FSM
// -----------------------------

enum Mode {
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
    Paused,
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
                    } => self.state = GameplayState::Paused,

                    _ => {}
                }
            }

            GameplayState::Paused => {
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
            GameplayState::Paused => {}
            GameplayState::Ending => {}
        }
    }

    fn draw(&mut self, frame: &mut DisplayCommands, scale: u8) -> Result<(), AnyError> {
        let game = &mut self.game;
        let snapshot = game.snapshot();
        let grid = &game.grid;

        draw_grid(frame, scale, &game.grid)?;
        draw_head(frame, scale, snapshot.head)?;
        draw_body(frame, scale, snapshot.body)?;
        draw_apple(frame, scale, snapshot.apple)?;

        if self.state == GameplayState::Paused {
            draw_overlay(frame, scale, grid)?;

            let center = Point::try_from(grid.size().scaled(scale / 2))?;
            // let above_center = center - (0, 32).into();
            // let below_center = center + (0, 32).into();

            let text = Text::builder("P A U S E D")
                .font(FONT_CALIPER_ID)
                .pt(24)
                .color(Color::WHITE)
                .position(center)
                .build();

            frame.text(&text)?;

            /*
            icon.blocks()
                .map(|block| block.translate(below_center))
                .try_for_each(|block| video.draw_block(&block))?;

            */
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

fn draw_body(draw: &mut DisplayCommands, scale: u8, body: &[Point]) -> Result<(), sdl3::Error> {
    body.iter().for_each(|cell| {
        draw.block(
            &Block::builder()
                .position(*cell)
                .scaled(scale)
                .rgb(80, 80, 80)
                .solid()
                .build(),
        )
        .unwrap();
    });
    Ok(())
}

fn draw_head(draw: &mut DisplayCommands, scale: u8, head: Point) -> Result<(), sdl3::Error> {
    draw.block(
        &Block::builder()
            .position(head)
            .scaled(scale)
            .rgb(80, 80, 80)
            .solid()
            .build(),
    )
}

fn draw_apple(draw: &mut DisplayCommands, scale: u8, apple: Point) -> Result<(), sdl3::Error> {
    draw.block(
        &Block::builder()
            .position(apple)
            .scaled(scale)
            .rgb(180, 80, 80)
            .solid()
            .build(),
    )
}

fn draw_grid(draw: &mut DisplayCommands, scale: u8, grid: &Grid) -> Result<(), sdl3::Error> {
    draw.block(
        &Block::builder()
            .rgb(120, 120, 120)
            .size((
                scale as i32 * grid.width() as i32,
                scale as i32 * grid.height() as i32,
            ))
            .build(),
    )
}

fn draw_overlay(draw: &mut DisplayCommands, scale: u8, grid: &Grid) -> Result<(), sdl3::Error> {
    draw.block(
        &Block::builder()
            .rgba(0, 0, 0, 120)
            .size((
                scale as i32 * grid.width() as i32,
                scale as i32 * grid.height() as i32,
            ))
            .build(),
    )
}

impl From<&Grid> for Rect {
    fn from(grid: &Grid) -> Self {
        Rect::new((0, 0), (grid.width(), grid.height()))
    }
}
