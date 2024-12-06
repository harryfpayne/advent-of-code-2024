use std::cmp::PartialEq;
use std::collections::HashSet;
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
    println!("{}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
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


    let mut all_obstacle_placements = vec![];
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = Point(y, x);
            if grid.get(&p) == &Cell::Empty {
                let mut grid_ = Grid::new(grid.grid.clone());
                grid_.set(&p, Cell::Obstacle);
                all_obstacle_placements.push(grid_);
            }
        }
    }


    let mut count = 0;
    for mut obstacle_placement in all_obstacle_placements {
        let mut guard_p_ = guard_p.clone();
        let mut visited: HashSet<(Point, Direction)> = HashSet::new();
        while step(&mut obstacle_placement, &mut guard_p_) {
            let d = obstacle_placement.get(&guard_p_);
            let Cell::Guard(dir) = d else {panic!("invalid guard point")};

            let current = (guard_p_, dir.clone());
            if visited.contains(&current) {
                count += 1;
                break
            }

            visited.insert(current);
        }
    }


    count
}

fn part_1(input: &str) -> usize {
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

    visited.len()
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