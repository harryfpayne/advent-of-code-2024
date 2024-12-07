use crate::coordinate::BCoord;

struct Grid<T> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    fn empty(height: usize, width: usize) -> Self
    where T : Clone + Default {
        let mut g = vec![vec![T::default(); width]; height];

        Grid { grid: g, height, width }
    }

    fn new(grid: Vec<Vec<T>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();
        Grid { grid, height, width }
    }

    fn get(&self, point: &BCoord) -> &T {
        &self.grid[point.y][point.x]
    }

    fn get_safe(&self, point: &BCoord) -> Option<&T> {
        self.grid.get(point.y)?.get(point.x)
    }

    fn get_mut(&mut self, point: &BCoord) -> &mut T {
        &mut self.grid[point.y][point.x]
    }

    fn get_safe_mut(&mut self, point: &BCoord) -> Option<&mut T> {
        self.grid.get_mut(point.y)?.get_mut(point.x)
    }

    fn set(&mut self, point: &BCoord, value: T) {
        self.grid[point.y][point.x] = value;
    }

    fn set_safe(&mut self, point: &BCoord, value: T) {
        if point.y >= self.height || point.x >= self.width {
            return
        }
        self.set(point, value);
    }

    fn find(&self, value: &T) -> Option<BCoord>
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
}