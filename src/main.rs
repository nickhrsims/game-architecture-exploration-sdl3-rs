use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::Canvas;
use sdl3::render::FRect;
use sdl3::video::Window;

use std::collections::VecDeque;
use std::vec::Vec;

pub mod grid;
use grid::Cell;
use grid::Grid;
use grid::Offset;

// -------------------------------------------------------------
// Game :: Model
// -------------------------------------------------------------

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl From<Direction> for Offset {
    fn from(value: Direction) -> Self {
        match value {
            Direction::North => Offset::NORTH,
            Direction::South => Offset::SOUTH,
            Direction::West => Offset::WEST,
            Direction::East => Offset::EAST,
        }
    }
}

struct Snake {
    vec: VecDeque<Cell>,
}

impl Snake {
    fn new(start: Cell) -> Self {
        let mut vec = VecDeque::new();
        vec.push_front(start);
        Self { vec }
    }

    fn head(&self) -> Cell {
        *self
            .vec
            .front()
            .expect("Snake body should always contain at least one cell")
    }

    fn body(&self) -> &[Cell] {
        let (cells, ..) = self.vec.as_slices();
        &cells[1..cells.len()]
    }

    fn next_head(&self, direction: Direction) -> Cell {
        self.head() + Offset::from(direction)
    }

    fn grow(&mut self, direction: Direction) {
        self.vec.push_front(self.next_head(direction));
        self.vec.make_contiguous();
    }

    fn step(&mut self, direction: Direction) {
        self.grow(direction);
        self.vec.pop_back();
    }
}

#[derive(PartialEq)]
enum GameState {
    Active,
    GameOver,
}

struct Game {
    state: GameState,
    grid: Grid,
    snake: Snake,
    apple: Cell,
    direction: Direction,
}

enum Mode {
    Normal,
}

impl Game {
    fn new(mode: Mode) -> Self {
        let state = GameState::Active;
        let direction = Direction::South;
        match mode {
            Mode::Normal => {
                let grid = Grid::new(48, 32);
                let snake = Snake::new(grid.random_cell());
                let apple = grid.random_cell();
                Self {
                    state,
                    grid,
                    snake,
                    apple,
                    direction,
                }
            }
        }
    }

    fn step(&mut self) {
        let state = &mut self.state;
        let grid = &self.grid;
        let snake = &mut self.snake;
        let apple = &mut self.apple;
        let direction = self.direction;

        match state {
            // Stepping GameOver is a no-op.
            GameState::GameOver => {}
            GameState::Active => {
                let next_head = snake.next_head(direction);
                if grid.contains(next_head) {
                    if next_head == *apple {
                        *apple = grid.random_cell();
                        snake.grow(direction)
                    } else {
                        snake.step(direction)
                    }
                } else {
                    *state = GameState::GameOver
                }
            }
        }
    }

    fn north(&mut self) {
        self.direction = Direction::North;
    }

    fn south(&mut self) {
        self.direction = Direction::South;
    }

    fn west(&mut self) {
        self.direction = Direction::West;
    }

    fn east(&mut self) {
        self.direction = Direction::East;
    }
}

// -------------------------------------------------------------
// SDL3 / Game Bridge
// -------------------------------------------------------------

struct Snapshot<'a> {
    apple: Cell,
    head: Cell,
    body: &'a [Cell],
}

trait Observable {
    fn snapshot(&self) -> Snapshot<'_>;
}

impl Observable for Game {
    fn snapshot(&self) -> Snapshot<'_> {
        let apple = self.apple;
        let head = self.snake.head();
        let body = self.snake.body();
        Snapshot { apple, head, body }
    }
}

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
        self.set_draw_color(Color::WHITE);
        self.fill_rects(&rects).expect("Rendering does not fail");
    }

    fn draw_head(&mut self, scale: u8, head: Cell) {
        let rect = FRect::new(
            head.0 as f32 * scale as f32,
            head.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );

        self.set_draw_color(Color::WHITE);
        self.fill_rect(rect).expect("Rendering does not fail");
    }

    fn draw_apple(&mut self, scale: u8, apple: Cell) {
        let rect = FRect::new(
            apple.0 as f32 * scale as f32,
            apple.1 as f32 * scale as f32,
            scale as f32,
            scale as f32,
        );
        self.set_draw_color(Color::RED);
        self.fill_rect(rect).expect("Rendering does not fail");
    }
}

fn clear_canvas(canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::BLACK);
    canvas.clear();
}

// -------------------------------------------------------------
// Main
// -------------------------------------------------------------

// TODO: Check head + body overlap
// TODO: Prevent apple spawn within snake
// TODO: Prevent reversing snake
// TODO: Alternating input queue system

fn main() {
    const TARGET_FPS: u8 = 60;
    const GRID_SCALE: u8 = 32;
    const SIM_FRAME: u8 = 10;

    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem
        .window("Snake", GRID_SCALE as u32 * 48, GRID_SCALE as u32 * 32)
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
    let mut frame_counter: u8 = 0;
    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut game = Game::new(Mode::Normal);

    while game.state == GameState::Active {
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

        frame_counter += 1;
        let apple = &game.apple;
        let head = &game.snake.head();
        eprintln!("{frame_counter} {head:?} {apple:?}");

        if frame_counter == SIM_FRAME {
            frame_counter = 0;
            game.step();
        }

        let snapshot = game.snapshot();
        clear_canvas(&mut canvas);
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

// -------------------------------------------------------------
// Tests
// -------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_cell_offset_normal() {
        let cell = Cell(1, 0);
        let offset = Offset::SOUTH;
        let result = cell + offset;
        eprintln!("{:?}", result);
        assert_eq!(result, Cell(1, 1));
    }

    #[test]
    fn sub_cell_from_cell() {
        let cell_a = Cell(3, 4);
        let cell_b = Cell(4, 5);
        assert_eq!(cell_b, cell_a + (cell_b - cell_a));

        let cell_c = Cell(3, 4);
        let cell_d = Cell(0, 0);
        assert_eq!(cell_d, cell_c + (cell_d - cell_c))
    }
}
