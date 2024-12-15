use std::collections::{HashMap, HashSet};
use text_io::scan;
use rand::Rng;

const INPUT: &str = include_str!("input.txt");
// const HEIGHT: i64 = 7;
// const WIDTH: i64 = 11;
const HEIGHT: i64 = 103;
const WIDTH: i64 = 101;

fn main() {
    println!("{:?}", part_2(INPUT));
}

fn part_2(input: &str) -> usize {
    type O = ((i64, i64), (i64, i64));

    fn print(robots: &Vec<O>) {
        let hs: HashSet<(i64, i64)> = HashSet::from_iter(robots.iter().map(|robot| robot.0));
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if hs.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    // entropy heuristic, looking for longest continuous run of filled spaces
    // based on the assumption that an xmas tree would have a flat base
    fn entropy(robots: &Vec<O>) -> f64 {
        let mut longest_continuous_row = 0;

        let mut grid = vec![vec![' '; WIDTH as usize]; HEIGHT as usize];

        for (robot, _) in robots.iter() {
            grid[robot.1 as usize][robot.0 as usize] = '#'
        }

        for row in grid {
            let mut curr = 0;
            for c in row {
                if curr > longest_continuous_row {
                    longest_continuous_row = curr;
                }

                if c == '#' {
                    curr += 1;
                } else {
                    curr = 0;
                }
            }
        }


        longest_continuous_row as f64
    }

    fn step(robots: &mut Vec<O>) {
        for (p, v) in robots {
            let d = (v.0 * 1, v.1 * 1);
            let p_next = (
                (p.0 + d.0).rem_euclid(WIDTH),
                (p.1 + d.1).rem_euclid(HEIGHT),
            );
            *p = p_next;
        }
    }

    let mut robots = parse(input);

    let mut highest_entropy = 0.0;
    let mut robots_at_highest_entropy = vec![];
    let mut steps = 0;
    for i in 0..10000 {
        let f = entropy(&robots);
        if f > highest_entropy {
            highest_entropy = f;
            robots_at_highest_entropy = robots.clone();
            steps = i
        }

        step(&mut robots);
    }

    print(&robots_at_highest_entropy);
    println!("{}", highest_entropy);

    steps
}

fn part_1(input: &str) -> usize {
    let robots = parse(input);
    let steps = 100;

    let mut quadrant_count = [0;4];
    let quadrant_lines = (WIDTH/2, HEIGHT/2);
    for (p, v) in robots {
        // println!("{:?} {:?}", p, v);
        let d = (v.0 * steps, v.1 * steps);
        // println!("Will move {:?}", d);
        let p_next = (
            (p.0 + d.0).rem_euclid(WIDTH),
            (p.1 + d.1).rem_euclid(HEIGHT),
        );

        // println!("{:?}", p_next);

        if p_next.0 > quadrant_lines.0 && p_next.1 > quadrant_lines.1 {
            quadrant_count[0] += 1;
        }
        if p_next.0 > quadrant_lines.0 && p_next.1 < quadrant_lines.1 {
            quadrant_count[1] += 1;
        }
        if p_next.0 < quadrant_lines.0 && p_next.1 > quadrant_lines.1 {
            quadrant_count[2] += 1;
        }
        if p_next.0 < quadrant_lines.0 && p_next.1 < quadrant_lines.1 {
            quadrant_count[3] += 1;
        }

        // println!("{:?}", quadrant_count);
        // println!();
    }

    quadrant_count.iter().fold(1, |a, b| a * b)
}

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64))> {
    input.lines().map(|line| {
        let mut p = (0, 0);
        let mut v = (0,0);
        scan!(line.bytes() => "p={},{} v={},{}", p.0, p.1, v.0, v.1);
        (p, v)
    }).collect()
}
