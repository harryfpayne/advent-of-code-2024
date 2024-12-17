use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;
use grid::grid::*;

mod grid;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1_and_2(INPUT));
    println!("{:?}", s.elapsed());
}

fn part_1_and_2(input: &str) -> (usize, usize) {
    let (grid, start) = parse(input);
    let mut start_path = HashSet::new();
    start_path.insert(start);

    let mut queue: BinaryHeap<QueueElement> = BinaryHeap::new();
    queue.push(QueueElement{
        score: 0,
        current_direction: Direction::R,
        position: start,
        path: start_path,
    });

    let mut visited: HashMap<(BCoord, Direction), i64> = HashMap::new();
    let mut min_score = i64::MAX;
    let mut path = HashSet::new();
    while let Some(curr) = queue.pop() {
        if let Some(c) = visited.get(&(curr.position, curr.current_direction)) {
            if c < &curr.score {
                continue;
            }
        }
        visited.insert((curr.position, curr.current_direction), curr.score);

        if grid.get(&curr.position) == &Cell::End && curr.score <= min_score {
            if curr.score < min_score {
                min_score = curr.score;
                path = curr.path.clone();
            } else if curr.score == min_score {
                path.extend(curr.path);
            }
            continue
        }

        for (dir, score_inc) in vec![
            (curr.current_direction, 1),
            (curr.current_direction.clockwise_90(), 1001),
            (curr.current_direction.anticlockwise_90(), 1001),
        ] {
            let next = curr.position.move_in(&dir);
            if next.is_none() { continue }
            let next = next.unwrap();
            if grid.get(&next) == &Cell::Wall { continue }

            let mut next_path = curr.path.clone();
            next_path.insert(next);

            queue.push(QueueElement {
                score: curr.score + score_inc,
                current_direction: dir,
                position: next,
                path: next_path,
            })
        }
    }

    (min_score as usize, path.len())
}


struct QueueElement {
    score: i64,
    current_direction: Direction,
    position: BCoord,
    path: HashSet<BCoord>,
}

impl PartialOrd<Self> for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.score.partial_cmp(&self.score)
    }
}
impl Ord for QueueElement {
    fn cmp(&self, other: &Self) -> Ordering { todo!() }
    fn max(self, other: Self) -> Self where Self: Sized { todo!() }
    fn min(self, other: Self) -> Self where Self: Sized { todo!() }
    fn clamp(self, min: Self, max: Self) -> Self where Self: Sized { todo!() }
}
impl Eq for QueueElement {}
impl PartialEq<Self> for QueueElement {
    fn eq(&self, other: &Self) -> bool { todo!() }
}

#[derive(Eq, PartialEq, Hash)]
enum Cell {
    Empty,
    Wall,
    End
}

fn parse(input: &str) -> (Grid<Cell>, BCoord) {
    let mut g = vec![];
    let mut start = (0,0);
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, ch) in line.chars().enumerate() {
            let a = match ch {
                '.' => Cell::Empty,
                '#' => Cell::Wall,
                'E' => Cell::End,
                'S' => {
                    start = (y,x);
                    Cell::Empty
                }
                _ => panic!(),
            };
            row.push(a);
        }
        g.push(row);
    }

    let c = BCoord::new(start.0, start.1, g.len(), g[0].len());
    (Grid::new(g), c)
}
