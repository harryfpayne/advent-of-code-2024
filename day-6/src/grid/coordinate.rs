use crate::direction::Direction;

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

    pub fn move_in(&self, direction: Direction) -> Option<BCoord> {
        if direction.imag > 0 && self.y == 0 || direction.real < 0 && self.x == 0 {
            return None
        }

        if direction.imag < 0 && self.y == self.y_bounds-1 || direction.real > 0 && self.x == self.x_bounds-1 {
            return None
        }

        Some(self.new_from(
            self.y - (direction.imag as usize),
            self.x + (direction.real as usize),
        ))
    }
}