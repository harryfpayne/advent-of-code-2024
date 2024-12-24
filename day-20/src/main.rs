use std::collections::HashMap;
use grid::grid::*;

mod grid;

const INPUT: &str = include_str!("input_test.txt");

fn main() {
    println!("{:?}", part_1(INPUT));
}

fn part_1(input: &str) -> (usize, usize) {
    let (grid, start, end) = parse(input);

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

    let distances = get_distances(&grid, &start, &end);

    let mut m = HashMap::new();
    let mut part1_count = 0;
    let mut part2_count = 0;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let p = BCoord::new(y, x, grid.height, grid.width);
            if *grid.get(&p) == false {
                continue
            }

            let d = distances.get(&p);
            if d.is_none() {
                panic!("space not on path {:?}", p)
            }
            let d = d.unwrap();

            for y in -40i32..=40 {
                for x in -40i32..=40 {
                    let y_ = p.y as i32 + y;
                    let x_ = p.x as i32 + x;
                    if y_ < 0 || y_ >= grid.height as i32 {
                        continue
                    }
                    if x_ < 0 || x_ >= grid.width as i32 {
                        continue
                    }

                    let n = p.new_from(y_ as usize , x_ as usize);
                    if !grid.get(&n) {
                        continue
                    }
                    let d1 = distances.get(&n).unwrap();
                    let dist = p.manhattan_distance(&n).abs();

                    let saved = *d1 - d - dist;

                    if saved < 50 || dist == 0 {
                        continue
                    }

                    if dist == 2 {
                        part1_count += 1
                    }
                    if dist <= 20 {
                        *m.entry(saved).or_insert(0) += 1;
                        part2_count += 1
                    }
                }
            }
        }
    }

    println!("{:?}", m);

    (part1_count, part2_count)
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
