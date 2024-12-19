#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
    pub height: usize,
    pub width: usize,
}

impl<T> Grid<T> {
    pub fn empty(height: usize, width: usize) -> Self
    where T : Clone + Default {
        let mut g = vec![vec![T::default(); width]; height];

        Grid { grid: g, height, width }
    }

    pub fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Grid { grid, height, width }
    }

    pub fn get(&self, point: &BCoord) -> &T {
        &self.grid[point.y][point.x]
    }

    pub fn get_safe(&self, point: &BCoord) -> Option<&T> {
        self.grid.get(point.y)?.get(point.x)
    }

    pub fn get_mut(&mut self, point: &BCoord) -> &mut T {
        &mut self.grid[point.y][point.x]
    }

    pub fn get_safe_mut(&mut self, point: &BCoord) -> Option<&mut T> {
        self.grid.get_mut(point.y)?.get_mut(point.x)
    }

    pub fn set(&mut self, point: &BCoord, value: T) {
        self.grid[point.y][point.x] = value;
    }

    pub fn set_safe(&mut self, point: &BCoord, value: T) {
        if point.y >= self.height || point.x >= self.width {
            return
        }
        self.set(point, value);
    }

    pub fn find(&self, value: &T) -> Option<BCoord>
    where T : PartialEq {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == *value {
                    return Some(BCoord::new(y, x, self.height, self.width));
                }
            }
        }
        None
    }
    pub fn find_all(&self, value: &T) -> Vec<BCoord>
    where T : PartialEq {
        let mut out = vec![];
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] == *value {
                    out.push(BCoord::new(y, x, self.height, self.width));
                }
            }
        }
        out
    }

    pub fn print(&self, p: fn(&T) -> char) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", p(&self.grid[y][x]));
            }
            println!();
        }
    }
}

use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Mul};

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug, Hash)]
pub struct Direction {
    pub(crate) real: i8,
    pub(crate) imag: i8,
}

impl Direction {
    pub fn new(real: i8, imag: i8) -> Direction {
        if real > 1 || real < -1 || imag > 1 || imag < -1 {
            panic!("Invalid direction {} {} ({} {} {} {})", real, imag, real > 1, real < -1, imag > 1, imag < -1);
        }
        Direction {
            real,
            imag
        }
    }

    pub const U: Direction = Direction{real: 0, imag: 1};
    pub const R: Direction = Direction{real: 1, imag: 0};
    pub const D: Direction = Direction{real: 0, imag: -1};
    pub const L: Direction = Direction{real: -1, imag: 0};
    pub const UR: Direction = Direction{real: 1, imag: 1};
    pub const DR: Direction = Direction{real: 1, imag: -1};
    pub const DL: Direction = Direction{real: -1, imag: -1};
    pub const UL: Direction = Direction{real: -1, imag: 1};

    pub const ORTHOGONAL: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];

    pub fn reflect(&self) -> Direction {
        Direction::new(-self.real, -self.imag)
    }

    pub fn clockwise_45(&self) -> Direction {
        self * Self::new(1, 1)
    }

    pub fn clockwise_90(&self) -> Direction {
        self * Self::new(0, 1)
    }

    pub fn anticlockwise_45(&self) -> Direction {
        self * Self::new(-1, -1)
    }

    pub fn anticlockwise_90(&self) -> Direction {
        self * Self::new(0, -1)
    }
}

impl Mul<Direction> for &Direction {
    type Output = Direction;

    fn mul(self, another: Direction) -> Direction {
        let real = (self.real * another.real) - (self.imag * another.imag);
        let imag = (self.real * another.imag) + (self.imag * another.real);

        Direction {
            real,
            imag
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match ((0i8).cmp(&self.real), 0i8.cmp(&self.imag)) {
            (Ordering::Less,    Ordering::Less) =>    write!(f, "DR"),
            (Ordering::Less,    Ordering::Equal) =>   write!(f, "R"),
            (Ordering::Less,    Ordering::Greater) => write!(f, "UR"),
            (Ordering::Equal,   Ordering::Less) =>    write!(f, "D"),
            (Ordering::Equal,   Ordering::Equal) =>   write!(f, "X"),
            (Ordering::Equal,   Ordering::Greater) => write!(f, "U"),
            (Ordering::Greater, Ordering::Less) =>    write!(f, "DL"),
            (Ordering::Greater, Ordering::Equal) =>   write!(f, "L"),
            (Ordering::Greater, Ordering::Greater) => write!(f, "UL"),
        }
    }
}


#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct BCoord {
    pub y: usize,
    pub x: usize,

    pub y_bounds: usize,
    pub x_bounds: usize,
}
impl BCoord {
    pub fn new(y: usize, x: usize, y_bounds: usize, x_bounds: usize) -> BCoord {
        BCoord { y, x, y_bounds, x_bounds }
    }

    pub fn new_from(&self, y: usize, x: usize) -> BCoord {
        BCoord { y, x, y_bounds: self.y_bounds, x_bounds: self.x_bounds }
    }

    pub fn orthogonal(&self) -> Vec<BCoord> {
        let mut out = vec![];
        if self.y != 0 {
            out.push(self.new_from(self.y - 1, self.x));
        }
        if self.x != 0 {
            out.push(self.new_from(self.y, self.x - 1));
        }
        if self.y + 1 < self.y_bounds {
            out.push(self.new_from(self.y + 1, self.x));
        }
        if self.x + 1 < self.x_bounds {
            out.push(self.new_from(self.y, self.x + 1));
        }
        out
    }

    pub fn adjacent(&self) -> Vec<BCoord> {
        let mut out = vec![];

        for y in -1..(1 as i32) {
            for x in -1..(1 as i32) {
                if y == 0 && x == 0 {
                    continue;
                }
                if self.y == 0 && y == 0 {
                    continue
                }
                if self.x == 0 && x == 0 {
                    continue
                }
                if self.y + (y as usize) >= self.y_bounds {
                    continue
                }
                if self.x + (x as usize) >= self.x_bounds {
                    continue
                }
                out.push(self.new_from(y as usize, x as usize));
            }
        }

        out
    }

    pub fn move_in(&self, direction: &Direction) -> Option<BCoord> {
        if direction.imag > 0 && self.y == 0 || direction.real < 0 && self.x == 0 {
            return None
        }

        if direction.imag < 0 && self.y == self.y_bounds-1 || direction.real > 0 && self.x == self.x_bounds-1 {
            return None
        }

        Some(self.new_from(
            (self.y as i64 - (direction.imag as i64)) as usize,
            (self.x as i64 + (direction.real as i64)) as usize,
        ))
    }
}