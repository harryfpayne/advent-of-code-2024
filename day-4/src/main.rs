mod grid;

use std::cmp::PartialEq;
use std::collections::HashSet;
use colored::Colorize;
use crate::grid::Direction;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part_2_convolution(INPUT));
}

fn part_1(input: &str) -> i32 {
    fn check_for_mas(grid: &grid::Grid<char>, p: grid::Point) -> i32 {
        let mut count = 0;
        'direction_loop: for d in grid::Direction::all() {

            let mut curr = p.clone();
            for char in vec!['M', 'A', 'S'] {
                let _curr = curr.dir(d, grid.grid.len());
                if _curr.is_none() { // I'm off the grid so change direction
                    continue 'direction_loop;
                }
                curr = _curr.unwrap();

                if grid.get(&curr) != &char {
                    continue 'direction_loop;
                }
            }
            count += 1
        }
        count
    }

    let grid: grid::Grid<char> = grid::Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut count = 0;
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            let p = grid::Point(x, y);

            if grid.get(&p) != &'X' {
                continue;
            }

            count += check_for_mas(&grid, p);
        }
    }

    count
}

fn part_2(input: &str) -> i32 {
    let mut pts = HashSet::new();
    fn check_for_x_mas(grid: &grid::Grid<char>, p: grid::Point, d: Direction) -> bool {
        let _scan = p.dir(d, grid.grid.len());
        if _scan.is_none() {
            return false;
        }
        let scan = _scan.unwrap();
        if grid.get(&_scan.unwrap()) != &'M' {
            return false;
        }
        // Find an M

        let opposite_scan = p.dir(d.reflection(), grid.grid.len());
        if opposite_scan.is_none() {
            return false;
        }
        // Get the other side of 'A'
        if grid.get(&opposite_scan.unwrap()) != &'S' {
            return false;
        }
        // Confirmed case of MAS

        let _scan_r = p.dir(d.clockwise().clockwise(), grid.grid.len());
        let _scan_l = p.dir(d.anticlockwise().anticlockwise(), grid.grid.len());

        if _scan_r.is_none() || _scan_l.is_none() {
            return false;
        }

        let scan_r = _scan_r.unwrap();
        let scan_l = _scan_l.unwrap();

        // Get the 2 points either side of M
        // One needs to be M, the other S.

        /*
            MMS
            SAM
            MSS
         */

        (grid.get(&scan_r) == &'M' && grid.get(&scan_l) == &'S') ||
            (grid.get(&scan_l) == &'M' && grid.get(&scan_r) == &'S')
    }

    fn check_for_mas(grid: &grid::Grid<char>, p: grid::Point) -> i32 {
        if grid.get(&p) != &'A' {
            return 0;
        }

        let mut count = 0;
        for d in Direction::all_orthogonal() {
            if check_for_x_mas(grid, p, d) {
                count += 1;
                break
            }
        }

        for d in Direction::all_diagonal() {
            if check_for_x_mas(grid, p, d) {
                count += 1;
                break
            }
        }

        count
    }

    let grid: grid::Grid<char> = grid::Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut count = 0;
    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            let p = grid::Point(x, y);
            if grid.get(&p) != &'A' {
                continue;
            }
            let c = check_for_mas(&grid, p);
            if c > 0 {
                pts.insert(p);
            }
            count += c
        }
    }

    for y in 0..grid.grid.len() {
        for x in 0..grid.grid[0].len() {
            let p = grid::Point(x, y);
            let c = grid.get(&p);
            if pts.contains(&p) {
                print!("{}", c.to_string().on_green())
            } else {
                print!("{}", c)
            }
        }
        print!("\n");
    }

    count
}

fn part_2_convolution(input: &str) -> i32 {
    const PATTERN_IGNORE_CHAR: char = '?';

    fn pattern_matches(grid: &grid::Grid<char>, pattern: &grid::Grid<char>, point: &grid::Point) -> bool {
        let y_range = point.0..point.0 + pattern.grid.len();
        let patter_range = 0..pattern.grid.len();

        for (gy, py) in y_range.zip(patter_range) {
            let x_range = point.1..point.1 + pattern.grid[0].len();
            let patter_range2 = 0..pattern.grid.len();

            for (gx, px) in x_range.zip(patter_range2) {
                if gy >= grid.grid.len() || gx >= grid.grid[0].len() {
                    return false;
                }
                let gp = grid::Point(gy, gx);
                let pp = grid::Point(py, px);

                if (grid.get(&gp) != pattern.get(&pp)) && pattern.get(&pp) != &PATTERN_IGNORE_CHAR {
                    return false;
                }
            }
        }
        true
    }

    let grid = grid::Grid::new(input.lines().map(|l| l.chars().collect()).collect());

    let mut patterns: Vec<grid::Grid<char>> = vec![];

    let mut d = grid::Direction::UL;
    let p = grid::Point(1, 1);
    for j in 0..4 {
        let mut g: Vec<Vec<char>> = vec![];
        for y in 0..=2 {
            let mut r = vec![];
            for x in 0..=2 {
                if y == 1 && x == 1 {
                    r.push('A')
                } else {
                    r.push(PATTERN_IGNORE_CHAR);
                }
            }
            g.push(r);
        }

        for i in 0..2 {
            let opposite_d = d.reflection();
            let m1 = p.dir(d, 3).unwrap();
            let s1 = p.dir(opposite_d, 3).unwrap();
            g[m1.0][m1.1] = 'M';
            g[s1.0][s1.1] = 'S';

            if i == 0 {
                d = d.clockwise().clockwise();
            }
        }
        patterns.push(grid::Grid::new(g));
    }

    let mut count = 0;
    for y in 0..grid.grid.len()-1 {
        for x in 0..grid.grid[0].len()-1 {
            let p = grid::Point(y, x);
            let pc = grid::Point(y+1, x+1);
            if grid.get(&pc) != &'A' {
                continue
            }

            for pattern in patterns.iter() {
                if pattern_matches(&grid, pattern, &p) {
                    count += 1
                }
            }
        }
    }

    count
}

// This one annoyed me, I couldn't figure out where I went wrong so implemented part 2 twice
// Ended up getting the same answer both times so looked on the subreddit
// formations in a + are not matches :(
// I spent way too long on this one, so I'm not going to clean up the code
// Only part_2_convolutions works
