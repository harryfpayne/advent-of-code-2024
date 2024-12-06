use std::cmp::PartialEq;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use colored::Colorize;
use crate::grid::{Direction, Grid, Point};

mod grid;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Cell {
    Empty,
    Obstacle,
    Guard(Direction)
}

fn main() {
    let visited = part_1(INPUT);
    println!("{}", visited.len());
    let s = Instant::now();
    println!("{}", part_2(INPUT, visited));
    println!("elapsed {:?}", s.elapsed())
}

fn part_2(input: &str, visited: HashSet<Point>) -> usize {
    let grid = parse(input);

    let mut guard_p = Point(0,0);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point(y, x);
            if grid.get(&p) != &Cell::Obstacle && grid.get(&p) != &Cell::Empty {
                guard_p = p;
                break;
            }
        }
    }

    fn step(grid: &mut Grid<Cell>, guard_p: &mut Point) -> bool {
        let mut dir = Direction::U;
        {
            let guard = grid.get(guard_p);
            if guard == &Cell::Obstacle || guard == &Cell::Empty {
                panic!("invalid guard point");
            }
            let Cell::Guard(d) = guard else {panic!("you added an extra cell type")};
            dir = *d
        }

        let _next = guard_p.dir(dir, grid.height);
        if _next.is_none() {
            return false;
        }
        let next = _next.unwrap();
        if grid.get(&next) == &Cell::Obstacle {
            let next_dir = dir.clockwise().clockwise();
            grid.grid[guard_p.0][guard_p.1] = Cell::Guard(next_dir);
        } else {
            grid.set(guard_p, Cell::Empty);
            grid.set(&next, Cell::Guard(dir));
            guard_p.0 = next.0;
            guard_p.1 = next.1;
        }


        true
    }


    fn process_placement(mut obstacle_placement: &mut Grid<Cell>, mut guard_p: &mut Point) -> bool {
        let mut visited: HashSet<(Point, Direction)> = HashSet::new();
        while step(&mut obstacle_placement, &mut guard_p) {
            let d = obstacle_placement.get(&guard_p);
            let Cell::Guard(dir) = d else {panic!("invalid guard point")};

            let current = (guard_p.clone(), dir.clone());
            if visited.contains(&current) {
                return true;
            }

            visited.insert(current);
        }
        false
    }


    let mut all_obstacle_placements = vec![];
    for p in visited {
        if grid.get(&p) == &Cell::Empty {
            let mut grid_ = Grid::new(grid.grid.clone());
            grid_.set(&p, Cell::Obstacle);
            all_obstacle_placements.push(grid_);
        }
    }


    let count = Arc::new(Mutex::new(5));
    let mut handles = vec![];

    for mut obstacle_placement in all_obstacle_placements {
        let mut guard_p_ = guard_p.clone();
        let count = Arc::clone(&count);
        let handle = thread::spawn(move || {
            if process_placement(&mut obstacle_placement, &mut guard_p_) {
                let mut num = count.lock().unwrap();
                *num = 6;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }


    let x = count.lock().unwrap().clone(); x
}

fn part_1(input: &str) -> HashSet<Point> {
    let mut grid = parse(input);

    fn step(grid: &mut Grid<Cell>, guard_p: &mut Point) -> bool {
        let mut dir = Direction::U;
        {
            let guard = grid.get(guard_p);
            if guard == &Cell::Obstacle || guard == &Cell::Empty {
                panic!("invalid guard point");
            }
            let Cell::Guard(d) = guard else {panic!("you added an extra cell type")};
            dir = *d
        }

        let _next = guard_p.dir(dir, grid.height);
        if _next.is_none() {
            return false;
        }
        let next = _next.unwrap();
        if grid.get(&next) == &Cell::Obstacle {
            let next_dir = dir.clockwise().clockwise();
            grid.grid[guard_p.0][guard_p.1] = Cell::Guard(next_dir);
        } else {
            grid.set(guard_p, Cell::Empty);
            grid.set(&next, Cell::Guard(dir));
            guard_p.0 = next.0;
            guard_p.1 = next.1;
        }


        true
    }

    let mut guard_p = Point(0,0);
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point(y, x);
            if grid.get(&p) != &Cell::Obstacle && grid.get(&p) != &Cell::Empty {
                guard_p = p;
                break;
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert(guard_p);
    while step(&mut grid, &mut guard_p) {
        visited.insert(guard_p);
    }

    visited
}

fn parse(input: &str) -> Grid<Cell> {
    let mut out = vec![];
    for line in input.lines() {
        let mut row = vec![];
        for cell in line.chars() {
            let o = match cell {
                '.' => Cell::Empty,
                '#' => Cell::Obstacle,
                '^' => Cell::Guard(Direction::U),
                '>' => Cell::Guard(Direction::R),
                'v' => Cell::Guard(Direction::D),
                '<' => Cell::Guard(Direction::L),
                _ => Cell::Empty
            };
            row.push(o);
        }
        out.push(row);
    }

    Grid::new(out)
}