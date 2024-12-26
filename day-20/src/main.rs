use std::collections::HashMap;
use std::time::Instant;
use grid::grid::*;

mod grid;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("{:?}", s.elapsed());
}

fn get_distances(
    grid: &Grid<bool>,
    start: &BCoord,
    end: &BCoord,
) -> HashMap<BCoord, i32> {
    let mut visited = HashMap::new();
    let mut next = *start;
    let mut distance = 0;

    while next != *end {
        visited.insert(next, distance);
        distance += 1;

        for adj in next.orthogonal() {
            if *grid.get(&adj) && !visited.contains_key(&adj) {
                next = adj;
                break
            }
        }
    }
    visited.insert(next, distance);

    visited
}

fn get_shortcuts(
    grid: &Grid<bool>,
    distance_map: &HashMap<BCoord, i32>,
    max_steps: i32,
    p: &BCoord,
) -> i32 {
    let d = distance_map.get(p);
    if !grid.get(&p) || d.is_none() {
        return 0
    }
    let d = d.unwrap();

    let mut count = 0;
    for dy in -max_steps..=max_steps {
        for dx in -max_steps..=max_steps {
            let shortcut_distance = dx.abs() + dy.abs();
            if shortcut_distance > max_steps || p.x as i32 + dx < 0 || p.y as i32 + dy < 0 {
                continue
            }
            let next = p.new_from((p.y as i32 + dy) as usize, (p.x as i32 + dx) as usize);
            if next.is_none() {
                continue
            }
            let next = next.unwrap();
            let next_d = distance_map.get(&next);
            if !grid.get(&next) || next_d.is_none() {
                continue
            }
            let next_d = next_d.unwrap();

            let distance_saved = next_d - d - shortcut_distance;
            if distance_saved >= 100 {
                count += 1;
            }
        }
    }

    count
}

fn part_2(input: &str) -> i32 {
    let (grid, start, end) = parse(input);
    let distance_map = get_distances(&grid, &start, &end);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = BCoord::new(y, x, grid.height, grid.width);
            if p == end {
                continue
            }

            let c = get_shortcuts(&grid, &distance_map, 20, &p);
            count += c;
        }
    }

    count
}

fn part_1(input: &str) -> i32 {
    let (grid, start, end) = parse(input);
    let distance_map = get_distances(&grid, &start, &end);

    let mut count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = BCoord::new(y, x, grid.height, grid.width);
            if p == end {
                continue
            }

            let c = get_shortcuts(&grid, &distance_map, 2, &p);
            count += c;
        }
    }

    count
}

fn parse(input: &str) -> (Grid<bool>, BCoord, BCoord) {
    let mut s = (0,0);
    let mut e = (0,0);

    let mut g = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                s = (y,x);
                row.push(true)
            } else if c == 'E' {
                e = (y,x);
                row.push(true)
            } else if c == '.' {
                row.push(true)
            } else {
                row.push(false)
            }
        }
        g.push(row)
    }

    let g = Grid::new(g);
    let s = BCoord::new(s.0, s.1, g.height, g.width);
    let e = BCoord::new(e.0, e.1, g.height, g.width);

    (g, s, e)
}
