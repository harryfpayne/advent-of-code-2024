use std::collections::{BinaryHeap, HashMap, HashSet};
use grid::grid::*;
use std::cmp::{Ordering, Reverse};
use std::time::Instant;

mod grid;

const INPUT: &str = include_str!("input.txt");
// const SIZE: usize = 6+1;
const SIZE: usize = 70+1;

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT, 1024));
    println!("{:?}", part_2(INPUT));
    println!("{:?}", s.elapsed());
}

fn part_2(input: &str) -> BCoord {
    let points = parse(input);
    let mut l = 0;
    let mut h = points.len();

    loop {
        if (h - l) <= 1 {
            return points[l];
        }

        let m = (h + l) / 2;
        if part_1(input, m).is_some() {
            l = m;
        } else {
            h = m;
        }
    }

}

fn part_1(input: &str, bytes_fallen: usize) -> Option<usize> {
    let points = parse(input);
    let mut grid = Grid::new(vec![vec![true; SIZE]; SIZE]);
    for p in points[..bytes_fallen].iter() {
        grid.set(p, false);
    }

    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    let start = BCoord::new(0,0,SIZE,SIZE);
    queue.push(QueueItem{p: start, d: 0});

    let mut min = usize::MAX;

    while let Some(curr) = queue.pop() {
        if let Some(distance) = visited.get(&curr.p) {
            if distance <= &curr.d {
                continue
            }
        }
        visited.insert(curr.p, curr.d);

        if curr.p.x == SIZE-1 && curr.p.y == SIZE-1 {
            min = curr.d;
            continue
        }

        for adj in curr.p.orthogonal() {
            if !*grid.get(&adj) {
                continue;
            }

            queue.push(QueueItem{p: adj, d: curr.d + 1});
        }
    }

    if min == usize::MAX {
        return None
    }

    Some(min)
}

struct QueueItem {
    p: BCoord,
    d: usize,
}

impl Eq for QueueItem {}

impl PartialEq<Self> for QueueItem {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for QueueItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.d.partial_cmp(&self.d)
    }
}

impl Ord for QueueItem {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized
    {
        todo!()
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized
    {
        todo!()
    }

    fn clamp(self, min: Self, max: Self) -> Self
    where
        Self: Sized
    {
        todo!()
    }
}


fn parse(input: &str) -> Vec<BCoord> {
    input.lines().map(|l| {
        let p = l.split_once(",").unwrap();
        BCoord::new(p.1.parse().unwrap(), p.0.parse().unwrap(), SIZE, SIZE)
    }).collect()
}
