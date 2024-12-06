use std::ops::Range;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Hash)]
pub struct Point(pub usize, pub usize);
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    U,R,D,L,
    UR,DR,UL,DL
}
impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![
            Direction::U,
            Direction::R,
            Direction::D,
            Direction::L,
            Direction::UR,
            Direction::DR,
            Direction::UL,
            Direction::DL,
        ]
    }
    pub fn all_orthogonal() -> Vec<Direction> {
        vec![
            Direction::U,
            Direction::R,
            Direction::D,
            Direction::L,
        ]
    }
    pub fn all_diagonal() -> Vec<Direction> {
        vec![
            Direction::UR,
            Direction::DR,
            Direction::UL,
            Direction::DL,
        ]
    }

    pub fn clockwise(&self) -> Direction {
        match self {
            Direction::U => Direction::UR,
            Direction::UR => Direction::R,
            Direction::R => Direction::DR,
            Direction::DR => Direction::D,
            Direction::D => Direction::DL,
            Direction::DL => Direction::L,
            Direction::L => Direction::UL,
            Direction::UL => Direction::U,
        }
    }
    pub fn anticlockwise(&self) -> Direction {
        match self {
            Direction::U => Direction::UL,
            Direction::UL => Direction::L,
            Direction::L => Direction::DL,
            Direction::DL => Direction::D,
            Direction::D => Direction::DR,
            Direction::DR => Direction::R,
            Direction::R => Direction::UR,
            Direction::UR => Direction::U,
        }
    }
    pub fn reflection(&self) -> Direction {
        self.clone()
            .clockwise()
            .clockwise()
            .clockwise()
            .clockwise()
    }
}


impl Point {
    pub fn orthogonal(&self, limit: usize) -> Vec<Point> {
        self.orthogonal_with_range((0..limit, 0..limit))
    }
    pub fn orthogonal_with_range(&self, range: (Range<usize>, Range<usize>)) -> Vec<Point> {
        let mut out = vec![];
        if self.0 - 1 >= range.0.start {
            out.push(Point(self.0 - 1, self.1));
        }
        if self.1 - 1 >= range.1.start {
            out.push(Point(self.0, self.1 - 1));
        }
        if self.0 + 1 < range.0.end {
            out.push(Point(self.0 + 1, self.1));
        }
        if self.1 + 1 < range.1.end {
            out.push(Point(self.0, self.1 + 1));
        }
        out
    }

    pub fn adjacent(&self, limit: usize) -> Vec<Point> {
        let mut out = vec![];

        for y in -1..(1 as i32) {
            for x in -1..(1 as i32) {
                if y == 0 && x == 0 {
                    continue;
                }
                if self.0 == 0 && y == 0 {
                    continue
                }
                if self.1 == 0 && x == 0 {
                    continue
                }
                if self.0 + (y as usize) == limit {
                    continue
                }
                if self.1 + (x as usize) == limit {
                    continue
                }
                out.push(Point(
                    (self.0 as i32 + y as i32) as usize,
                    (self.1 as i32 + x as i32) as usize
                ));
            }
        }

        out
    }

    pub fn dir(&self, d: Direction, limit: usize) -> Option<Point> {
        if self.0 == 0 && (d == Direction::U || d == Direction::UL || d == Direction::UR) {
            return None;
        }
        if self.1 == 0 && (d == Direction::L || d == Direction::UL || d == Direction::DL) {
            return None;
        }

        let next = match d {
            Direction::U =>  (self.0 - 1, self.1),
            Direction::UL => (self.0 - 1, self.1 - 1),
            Direction::L =>  (self.0, self.1 - 1),
            Direction::DL => (self.0 + 1, self.1 - 1),
            Direction::D =>  (self.0 + 1, self.1),
            Direction::DR => (self.0 + 1, self.1 + 1),
            Direction::R =>  (self.0, self.1 + 1),
            Direction::UR => (self.0 - 1, self.1 + 1),
        };

        if next.0 >= limit || next.1 >= limit {
            return None;
        }
        Some(Point(next.0 as usize, next.1 as usize))
    }

    pub fn dir_n(&self, d: Direction, n: i32, limit: usize) -> Option<Point> {
        let mut next = Point(self.0, self.1);
        for _ in 0..n {
            let p = next.dir(d, limit);
            if p.is_none() {
                return None
            }
        }
        return Some(next);
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(v: Vec<Vec<T>>) -> Grid<T> {
        Grid { grid: v }
    }

    pub fn get(&self, p: &Point) -> &T {
        &self.grid[p.0][p.1]
    }
}

