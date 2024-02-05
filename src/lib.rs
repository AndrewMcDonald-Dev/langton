use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub struct World {
    pub cells: HashMap<Position, Cell>,
    ant: Ant,
}

impl World {
    pub fn get_color(&self, pos: Position) -> Color {
        match self.cells.get(&pos) {
            Some(cell) => cell.color,
            None => Color::White,
        }
    }
}

impl Default for World {
    fn default() -> Self {
        World {
            cells: HashMap::new(),
            ant: Ant::from(Position(0, 0)),
        }
    }
}

impl Iterator for World {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        let old_self = self.clone();

        // Langton Ant Logic
        let cur_pos = self.ant.pos;
        let cur_cell = self.cells.get_mut(&cur_pos);
        if let Some(cell) = cur_cell {
            match cell.color {
                Color::White => {
                    //Never Runs
                    self.ant.turn_right();
                    cell.color = Color::Black;
                }
                Color::Black => {
                    //When cell is set to white it is removed from hashmap
                    self.ant.turn_left();
                    self.cells.remove(&cur_pos);
                    // cell.color = Color::White;
                }
            }
        } else {
            self.ant.turn_right();
            self.cells.insert(cur_pos, Cell::new(cur_pos, Color::Black));
        }

        if &old_self == self {
            None
        } else {
            Some(self.clone())
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Ant {
    pos: Position,
    dir: Direction,
}

impl Ant {
    fn turn_left(&mut self) {
        let diff = match self.dir {
            Direction::North => Position(-1, 0),
            Direction::East => Position(0, 1),
            Direction::South => Position(1, 0),
            Direction::West => Position(0, -1),
        };

        self.pos.mov(diff);

        self.dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
    }
    fn turn_right(&mut self) {
        let diff = match self.dir {
            Direction::North => Position(1, 0),
            Direction::East => Position(0, -1),
            Direction::South => Position(-1, 0),
            Direction::West => Position(0, 1),
        };

        self.pos.mov(diff);

        self.dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
    }
}

impl From<Position> for Ant {
    fn from(value: Position) -> Self {
        Ant {
            pos: value,
            dir: Direction::North,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position(pub i32, pub i32);

impl Position {
    pub fn mov(&mut self, diff: Position) {
        (self.0, self.1) = (self.0 + diff.0, self.1 + diff.1);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn to_values(&self) -> [f32; 4] {
        match self {
            Color::White => [1.0, 1.0, 1.0, 1.0],
            Color::Black => [0.0, 0.0, 0.0, 1.0],
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Cell {
    pub pos: Position,
    color: Color,
}

impl Cell {
    fn new(pos: Position, color: Color) -> Self {
        Cell { pos, color }
    }
}
