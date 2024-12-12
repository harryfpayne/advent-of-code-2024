use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::Instant;
use crate::grid::*;
use crate::coordinate::*;
use crate::direction::*;

mod grid;
mod coordinate;
mod direction;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("{:?}", s.elapsed());
}


fn part_2(input: &str) -> usize {
    fn side_pass(
        visited: &HashMap<BCoord, usize>,
        grid: &Grid<char>,
    ) -> HashMap<usize, usize> {
        let mut side_count: HashMap<usize, usize> = HashMap::new();
        for (y, row) in grid.grid.iter().enumerate() {
            /*
                Does a scan through each row
                keeps track of if the last cell was on an edge above (U) and or below (D)
                If it one of those were an edge and isn't now it means we've found a complete side

                Simple grid with corresponding indexes to simplify referencing cells
                EEE -> 012
                EXE -> 345

                I'm passing through the first row, I look at 1. It is on an edge in both U and D
                I then got to 2, it's still missing on U but not D
                Because D is no longer an edge it means I've seen a complete edge so ++ the count
             */


            let mut u_was_missing_previously = false;
            let mut d_was_missing_previously = false;
            let mut previous_zone = 0;

            for (x, c) in row.iter().enumerate() {
                let p = BCoord::new(y, x, grid.grid.len(), row.len());
                let p_zone = visited.get(&p).unwrap();

                if *p_zone != previous_zone {
                    if u_was_missing_previously {
                        side_count.entry(previous_zone).and_modify(|v| *v += 1).or_insert(1);
                    }
                    if d_was_missing_previously {
                        side_count.entry(previous_zone).and_modify(|v| *v += 1).or_insert(1);
                    }
                    u_was_missing_previously = false;
                    d_was_missing_previously = false;
                }

                let mut u_is_missing = false;
                let mut d_is_missing = false;

                let up = p.move_in(Direction::U);
                if up.is_none() || visited.get(&up.unwrap()).unwrap() != p_zone {
                    u_is_missing = true;
                }

                let down = p.move_in(Direction::D);
                if down.is_none() || visited.get(&down.unwrap()).unwrap() != p_zone {
                    d_is_missing = true;
                }

                if u_was_missing_previously && !u_is_missing {
                    side_count.entry(*p_zone).and_modify(|v| *v += 1).or_insert(1);
                }
                if d_was_missing_previously && !d_is_missing {
                    side_count.entry(*p_zone).and_modify(|v| *v += 1).or_insert(1);
                }

                u_was_missing_previously = u_is_missing;
                d_was_missing_previously = d_is_missing;
                previous_zone = *p_zone;
            }

            if u_was_missing_previously {
                side_count.entry(previous_zone).and_modify(|v| *v += 1).or_insert(1);
            }
            if d_was_missing_previously {
                side_count.entry(previous_zone).and_modify(|v| *v += 1).or_insert(1);
            }
        }

        side_count
    }

    fn flood_fill(
        visited: &mut HashMap<BCoord, usize>,
        grid: &Grid<char>,
        p: BCoord,
        current_zone: usize
    ) {
        if visited.contains_key(&p) {
            return
        }

        visited.insert(p, current_zone);

        let c = grid.get(&p);
        for orth in p.orthogonal() {
            if grid.get(&orth) != c {
                continue
            }
            flood_fill(visited, grid, orth, current_zone);
        }
    }

    let grid = parse(input);

    let mut visited = HashMap::new();
    let mut current_zone = 0;
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let p = BCoord::new(y, x, grid.grid.len(), row.len());

            let num_visited = visited.len();
            flood_fill(&mut visited, &grid, p, current_zone);
            if visited.len() != num_visited {
                current_zone += 1;
            }
        }
    }


    let mut area_map: HashMap<usize, usize> = HashMap::new();
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let p = BCoord::new(y, x, grid.grid.len(), row.len());
            let p_zone = visited.get(&p).unwrap();
            area_map.entry(*p_zone).and_modify(|v| *v += 1).or_insert(1);
        }
    }

    let side_count = side_pass(
        &visited,
        &grid,
    );
    let rotated_grid = grid.rotate_clockwise();
    let side_count2 = side_pass(
        &visited,
        &rotated_grid,
    );

    area_map.iter().fold(0, |acc, (k, v)| {
        let a = side_count.get(k).unwrap();
        let b = side_count2.get(k).unwrap();
        acc + v * (a+b)
    })
}


fn part_1(input: &str) -> usize {
    fn flood_fill(
        visited: &mut HashMap<BCoord, usize>,
        grid: &Grid<char>,
        p: BCoord,
        current_zone: usize
    ) {
        if visited.contains_key(&p) {
            return
        }

        visited.insert(p, current_zone);

        let c = grid.get(&p);
        for orth in p.orthogonal() {
            if grid.get(&orth) != c {
                continue
            }
            flood_fill(visited, grid, orth, current_zone);
        }
    }

    let grid = parse(input);

    let mut visited = HashMap::new();
    let mut current_zone = 0;
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let p = BCoord::new(y, x, grid.grid.len(), row.len());

            let num_visited = visited.len();
            flood_fill(&mut visited, &grid, p, current_zone);
            if visited.len() != num_visited {
                current_zone += 1;
            }
        }
    }


    let mut area_map: HashMap<usize, usize> = HashMap::new();
    let mut perimeter_map: HashMap<usize, usize> = HashMap::new();
    for (y, row) in grid.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            let p = BCoord::new(y, x, grid.grid.len(), row.len());
            let p_zone = visited.get(&p).unwrap();

            area_map.entry(*p_zone).and_modify(|v| *v += 1).or_insert(1);

            let mut edges = 4;
            for orth in p.orthogonal() {
                edges -= 1;
                let o_zone = visited.get(&orth).unwrap();
                if p_zone != o_zone {
                    perimeter_map.entry(*p_zone)
                        .and_modify(|v| *v += 1)
                        .or_insert(1);
                }
            }
            perimeter_map.entry(*p_zone)
                .and_modify(|v| *v += edges)
                .or_insert(edges);
        }
    }

    // println!("{:?}", perimeter_map);
    // println!("{:?}", area_map);
    // for (y, row) in grid.grid.iter().enumerate() {
    //     for (x, c) in row.iter().enumerate() {
    //         let p = BCoord::new(y, x, grid.grid.len(), row.len());
    //         print!("{}", visited.get(&p).unwrap());
    //     }
    //     println!()
    // }

    perimeter_map.iter().fold(0, |acc, (k, v)| {
        let a = area_map.get(k).unwrap();
        acc + v * a
    })
}

fn parse(input: &str) -> Grid<char> {
    let g = input.lines().map(|l| l.chars().collect()).collect();
    Grid::new(g)
}

fn print(g: &Grid<char>) {
    for (y, row) in g.grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            print!("{}", c);
        }
        println!();
    }
}