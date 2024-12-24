use std::time::Instant;
use crate::grid::grid::{BCoord, Direction, Grid};
use crate::part_1::part_1;

mod grid;
mod part_1;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("{:?}", s.elapsed())
}

fn part_2(input: &str) -> usize {
    let (mut grid, mut directions) = parse(input);

    fn move_to(grid: &mut Grid<Cell>, curr: &BCoord, next: &BCoord) {
        grid.set(next, *grid.get(&curr));
        grid.set(curr, Cell::Empty);
    }

    fn move_boxes(
        grid: &mut Grid<Cell>,
        current_position: &BCoord,
        direction: &Direction,
        edit_mode: bool,
    ) -> bool {
        let next = current_position.move_in(direction);
        if next.is_none() {
            return false
        }
        let next = next.unwrap();

        let next_v = grid.get(&next);
        if next_v == &Cell::Wall {
            return false;
        }
        if next_v != &Cell::BoxR && next_v != &Cell::BoxL {
            if edit_mode {
                move_to(grid, current_position, &next);
            }
            return true;
        }

        if next_v == &Cell::BoxR {
            let can_move;
            if direction == &Direction::L || direction == &Direction::R {
                can_move = move_boxes(grid, &next, direction, edit_mode);
            } else {
                let can_i_move = move_boxes(grid, &next, direction, edit_mode);
                let can_other_move = move_boxes(grid, &next.move_in(&Direction::L).unwrap(), direction, edit_mode);
                can_move = can_i_move && can_other_move;
            }
            if edit_mode && can_move {
                move_to(grid, current_position, &next);
            }
            return can_move
        }

        if next_v == &Cell::BoxL {
            let can_move;
            if direction == &Direction::L || direction == &Direction::R {
                can_move = move_boxes(grid, &next, direction, edit_mode);
            } else {
                let can_i_move = move_boxes(grid, &next, direction, edit_mode);
                let can_other_move = move_boxes(grid, &next.move_in(&Direction::R).unwrap(), direction, edit_mode);
                can_move = can_i_move && can_other_move;
            }
            if edit_mode && can_move {
                move_to(grid, current_position, &next);
            }
            return can_move;
        }

        false
    }


    let mut current_position = grid.find(&Cell::Robot).unwrap();
    for direction in directions {
        let want_to_move_to = current_position.move_in(&direction).unwrap();
        let in_next_cell = grid.get(&want_to_move_to);
        match in_next_cell {
            Cell::Empty => {
                move_to(&mut grid, &current_position, &want_to_move_to);
                current_position = want_to_move_to;
            },
            Cell::Wall => {},
            Cell::BoxL => {
                if move_boxes(&mut grid, &current_position, &direction, false) {
                    move_boxes(&mut grid, &current_position, &direction, true);
                    current_position = want_to_move_to;
                }
            },
            Cell::BoxR => {
                if move_boxes(&mut grid, &current_position, &direction, false) {
                    move_boxes(&mut grid, &current_position, &direction, true);
                    current_position = want_to_move_to;
                }
            },
            Cell::Robot => panic!("trying to move to robot"),
        };
    }


    grid.grid.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(acc, move |acc, (x, cell)| {
            if cell != &Cell::BoxL { acc }
            else {
                acc + (y*100) + x
            }
        })
    })
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Cell {
    Empty,
    Wall,
    Robot,
    BoxL,
    BoxR,
}

fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let (g, d) = input.split_once("\n\n").unwrap();

    let g = g.lines().map(|l|
        l.chars().flat_map(|c| match c {
            '.' => [Cell::Empty, Cell::Empty],
            '#' => [Cell::Wall, Cell::Wall],
            '@' => [Cell::Robot, Cell::Empty],
            'O' => [Cell::BoxL, Cell::BoxR],
            _ => panic!(),
        }).collect::<Vec<Cell>>()
    ).collect::<Vec<_>>();

    let d = d.replace("\n", "").chars().map(|c| match c {
        '<' => Direction::L,
        '>' => Direction::R,
        '^' => Direction::U,
        'v' => Direction::D,
        _ => panic!(),
    }).collect::<Vec<Direction>>();

    (Grid::new(g), d)
}