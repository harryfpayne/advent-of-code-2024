use std::cmp::Ordering;
use std::fmt::{Display, Error, Formatter};
use std::ops::{Mul};

#[derive(PartialEq, PartialOrd, Eq, Copy, Clone, Debug)]
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


