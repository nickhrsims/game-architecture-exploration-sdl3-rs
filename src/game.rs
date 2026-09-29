use std::collections::VecDeque;
use std::vec::Vec;

use crate::grid::{Grid, Offset, Point};

pub struct Game {
    pub state: GameState,
    pub grid: Grid,
    snake: Snake,
    apple: Point,
    direction: Direction,
    queue: VecDeque<Direction>,
}

#[derive(PartialEq, Copy, Clone)]
pub struct Snapshot<'a> {
    pub apple: Point,
    pub head: Point,
    pub body: &'a [Point],
    pub state: GameState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum GameState {
    Playing,
    GameOver,
}

struct Snake {
    head: Point,
    body: Vec<Point>,
    tail: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Game {
    pub fn new(grid_width: u16, grid_height: u16) -> Self {
        if grid_width < 16 || grid_height < 16 {
            panic!("Grid width and height must each be at least 16");
        }

        let state = GameState::Playing;
        let direction = Direction::East;
        let queue = VecDeque::new();
        let grid = Grid::new(grid_width, grid_height);
        let apple = Point::new(grid_width as i32 - 4, grid_height as i32 - 4);
        let mut snake = Snake::new(Point::new(4, 4));
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

    pub fn snapshot(&self) -> Snapshot<'_> {
        let apple = self.apple;
        let head = self.snake.head;
        let body = self.snake.body.as_slice();
        let state = self.state;
        Snapshot {
            apple,
            head,
            body,
            state,
        }
    }

    pub fn step(&mut self) {
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
                            *apple = grid.random_point();
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

    pub fn north(&mut self) {
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
    pub fn south(&mut self) {
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

    pub fn west(&mut self) {
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

    pub fn east(&mut self) {
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

impl Snake {
    fn new(head: Point) -> Self {
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

#[cfg(test)]
mod tests {}
