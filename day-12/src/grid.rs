use crate::coordinate::BCoord;
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


    pub fn rotate_clockwise(&self) -> Grid<T>
    where T : Clone {
        let mut out = self.grid.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                out[y][x] = self.grid[y][x].clone();
            }
        }


        Grid::new(out)
    }
}

