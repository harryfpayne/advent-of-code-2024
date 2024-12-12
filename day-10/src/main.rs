use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;
use crate::grid::*;
use crate::coordinate::*;

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

fn part_1(input: &str) -> usize {
    fn bfs(grid: &Grid<i32>, start: BCoord) -> usize {
        let mut queue: VecDeque<BCoord> = VecDeque::new();
        queue.push_back(start.clone());

        let mut visited = HashSet::new();
        let mut summits = 0;
        while let Some(p) = queue.pop_front() {
            let height = grid.get(&p);


            if visited.contains(&p) {
                continue;
            }
            visited.insert(p.clone());

            if height == &9 {
                summits += 1;
                continue;
            }

            for next in p.orthogonal() {
                if *grid.get(&next) == height + 1 {
                    queue.push_back(next)
                }
            }
        }

        summits
    }



    let grid = parse(input);
    let trailheads = grid.find_all(&0);

    let mut sum = 0;
    for trailhead in trailheads {
        sum += bfs(&grid, trailhead)
    }

    sum
}

fn part_2(input: &str) -> usize {
    fn bfs(grid: &Grid<i32>, start: BCoord) -> usize {
        let mut queue: VecDeque<(BCoord, BCoord)> = VecDeque::new();
        //                      current        parent
        queue.push_back((start.clone(), start.clone()));

        let mut visited = HashMap::new();

        let mut summits = 0;
        while let Some(p) = queue.pop_front() {
            let height = grid.get(&p.0);

            if height == &9 {
                let trails_here = visited.get(&p.1).unwrap();
                summits += trails_here;
                continue;
            }

            if visited.contains_key(&p.0) {
                let trails_here = visited.get(&p.1).unwrap();
                *visited.entry(p.0.clone()).or_insert(0) += *trails_here;
                continue;
            }
            visited.insert(p.0.clone(), *visited.get(&p.1).unwrap_or(&1));

            for next in p.0.orthogonal() {
                if *grid.get(&next) == height + 1 {
                    queue.push_back((next, p.0))
                }
            }
        }

        summits
    }



    let grid = parse(input);
    let trailheads = grid.find_all(&0);

    let mut sum = 0;
    for trailhead in trailheads {
        sum += bfs(&grid, trailhead)
    }

    sum
}

fn parse(input: &str) -> Grid<i32> {
    let grid = input.lines().map(
        |l| l.chars().map(|c| c.to_string().parse().unwrap_or(-2)).collect()
    ).collect();

    Grid::new(grid)
}
