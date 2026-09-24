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
    head: Cell,
    body: Vec<Cell>,
    tail: usize,
}

impl Snake {
    fn new(head: Cell) -> Self {
        let body = Vec::new();
        Self {
            head,
            body,
            tail: 0,
        }
    }

    fn step(&mut self, direction: Direction) {
        if !self.body.is_empty() {
            self.body[self.tail] = self.head;
            self.tail += 1;
        }

        self.head += Offset::from(direction);

        if self.tail == self.body.len() {
            self.tail = 0;
        }
    }

    fn grow(&mut self, direction: Direction) {
        self.body.insert(
            self.tail,
            if self.body.is_empty() {
                self.head
            } else {
                self.body[self.tail]
            },
        );
        self.step(direction);
    }
}

#[derive(PartialEq)]
enum GameState {
    Playing,
    GameOver,
}

struct Game {
    state: GameState,
    grid: Grid,
    snake: Snake,
    apple: Cell,
    direction: Direction,
    queue: VecDeque<Direction>,
}

enum Mode {
    Normal,
}

impl Game {
    fn new(mode: Mode) -> Self {
        let state = GameState::Playing;
        let direction = Direction::East;
        let queue = VecDeque::new();
        match mode {
            Mode::Normal => {
                let grid = Grid::new(24, 24);
                let apple = Cell::new(18, 18);
                let mut snake = Snake::new(Cell::new(4, 4));
                snake.grow(Direction::East);
                snake.grow(Direction::East);
                snake.grow(Direction::East);
                Self {
                    state,
                    grid,
                    snake,
                    apple,
                    direction,
                    queue,
                }
            }
        }
    }

    fn step(&mut self) {
        let state = &mut self.state;
        let grid = &self.grid;
        let snake = &mut self.snake;
        let apple = &mut self.apple;
        let direction = &mut self.direction;
        let queue = &mut self.queue;

        match state {
            // Stepping GameOver is a no-op.
            GameState::GameOver => {}
            GameState::Playing => {
                let next_head = snake.head + Offset::from(*direction);
                if grid.contains(next_head) {
                    if next_head == *apple {
                        loop {
                            *apple = grid.random_cell();
                            if *apple != snake.head && !snake.body.contains(apple) {
                                break;
                            }
                        }
                        snake.grow(*direction);
                    } else {
                        snake.step(*direction);
                    }
                    if snake.body.contains(&snake.head) {
                        *state = GameState::GameOver;
                    }
                } else {
                    *state = GameState::GameOver;
                }
                if let Some(new_direction) = queue.pop_back() {
                    *direction = new_direction;
                }
            }
        }
    }

    fn north(&mut self) {
        let len = self.queue.len();
        if len <= 3 {
            let direction = match self.queue.front() {
                Some(direction) => *direction,
                None => self.direction,
            };

            match direction {
                Direction::West | Direction::East => {
                    self.queue.push_front(Direction::North);
                }
                _ => {}
            }
        }
    }
    fn south(&mut self) {
        let len = self.queue.len();
        if len <= 3 {
            let direction = match self.queue.front() {
                Some(direction) => *direction,
                None => self.direction,
            };

            match direction {
                Direction::West | Direction::East => {
                    self.queue.push_front(Direction::South);
                }
                _ => {}
            }
        }
    }

    fn west(&mut self) {
        let len = self.queue.len();
        if len <= 3 {
            let direction = match self.queue.front() {
                Some(direction) => *direction,
                None => self.direction,
            };

            match direction {
                Direction::North | Direction::South => {
                    self.queue.push_front(Direction::West);
                }
                _ => {}
            }
        }
    }

    fn east(&mut self) {
        let len = self.queue.len();
        if len <= 3 {
            let direction = match self.queue.front() {
                Some(direction) => *direction,
                None => self.direction,
            };

            match direction {
                Direction::North | Direction::South => {
                    self.queue.push_front(Direction::East);
                }
                _ => {}
            }
        }
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
        let head = self.snake.head;
        let body = self.snake.body.as_slice();
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
