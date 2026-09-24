// -------------------------------------------------------------
// Cell
// -------------------------------------------------------------

#[derive(Default, Copy, Clone, Eq, PartialEq, Debug)]
pub struct Cell(pub i32, pub i32);

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn zero() -> Self {
        Self(0, 0)
    }

    pub fn random(x_limit: i32, y_limit: i32) -> Self {
        Self(
            rand::random_range(0..x_limit),
            rand::random_range(0..y_limit),
        )
    }
}

// -------------------------------------------------------------
// Offset
// -------------------------------------------------------------

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Offset(i32, i32);

#[derive(Debug, Eq, PartialEq)]
pub struct Grid {
    width: u16,
    height: u16,
}

impl Offset {
    pub const NORTH: Offset = Offset(0, -1);
    pub const SOUTH: Offset = Offset(0, 1);
    pub const WEST: Offset = Offset(-1, 0);
    pub const EAST: Offset = Offset(1, 0);
}

impl Default for Offset {
    fn default() -> Self {
        Offset::SOUTH
    }
}

// -------------------------------------------------------------
// Grid
// -------------------------------------------------------------

impl Grid {
    pub fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }
    pub fn contains(&self, cell: Cell) -> bool {
        let before = cell.0 < 0;
        let after = cell.0 >= self.width as i32;
        let above = cell.1 < 0;
        let below = cell.1 >= self.height as i32;
        !(before || after || above || below)
    }
    pub fn random_cell(&self) -> Cell {
        Cell::random(self.width as i32, self.height as i32)
    }
}

// -------------------------------------------------------------
// Cell & Offset :: Operators
// -------------------------------------------------------------

impl std::ops::Add<Offset> for Cell {
    type Output = Cell;
    fn add(self, rhs: Offset) -> Cell {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0.checked_add(rhs0).expect("lhs + rhs within bounds");
        let r1 = lhs1.checked_add(rhs1).expect("lhs + rhs within bounds");

        Cell(r0, r1)
    }
}

impl std::ops::Sub for Cell {
    type Output = Offset;
    fn sub(self, rhs: Cell) -> Offset {
        let lhs0 = self.0;
        let lhs1 = self.1;
        let rhs0 = rhs.0;
        let rhs1 = rhs.1;

        let r0 = lhs0 - rhs0;
        let r1 = lhs1 - rhs1;

        Offset(r0, r1)
    }
}

// -------------------------------------------------------------
// Tests
// -------------------------------------------------------------

#[cfg(test)]
mod tests {
    use crate::Cell;
    use crate::Grid;
    #[test]
    fn grid_contains() {
        let grid = Grid::new(16, 16);
        let inside = Cell::new(4, 12);
        let outside = Cell::new(-2, 6);
        assert!(grid.contains(inside));
        assert!(!grid.contains(outside));
    }
}
