use crate::grid::grid::{BCoord, Direction, Grid};

mod grid;
mod part_1;

const INPUT: &str = include_str!("input_test.txt");

fn main() {
    println!("{:?}", part_2(INPUT));
}

fn part_2(input: &str) {
    let (mut grid, directions) = parse(input);

    fn move_to(grid: &mut Grid<Cell>, curr: &BCoord, next: &BCoord) {
        grid.set(next, *grid.get(&curr));
        grid.set(curr, Cell::Empty);
    }

    fn can_move_boxes_rec(grid: &mut Grid<Cell>, curr: &BCoord, dir: &Direction, ignore_partner: bool) -> bool {
        // println!("Looking at {} {}", curr.y, curr.x);
        let next = curr.move_in(dir).unwrap();
        let v = grid.get(&next);
        // println!("Want to move to ({} {}) {:?}", next.y, next.x, v);
        match v {
            &Cell::Empty => {
                // println!("It's empty moving");
                true
            },
            &Cell::Wall => {
                // println!("It's wall");
                false
            },
            &Cell::BoxL => {
                if dir == &Direction::R || dir == &Direction::L {
                    let can_move = can_move_boxes_rec(grid, &next, dir, false);
                    can_move
                } else if ignore_partner {
                    return can_move_boxes_rec(grid, &next, dir, false);
                } else {
                    let can_i_move = can_move_boxes_rec(grid, &next, dir, false);
                    let curr_partner = curr.move_in(&Direction::R).unwrap();
                    let can_partner_move = can_move_boxes_rec(grid, &curr_partner, dir, true);
                    return can_i_move && can_partner_move
                }
            },
            &Cell::BoxR => {
                if dir == &Direction::R || dir == &Direction::L {
                    let can_move = can_move_boxes_rec(grid, &next, dir, false);
                    can_move
                } else if ignore_partner {
                    return can_move_boxes_rec(grid, &next, dir, false);
                } else {
                    let can_i_move = move_boxes_rec(grid, &next, dir, false);
                    let curr_partner = curr.move_in(&Direction::L).unwrap();
                    let can_partner_move = can_move_boxes_rec(grid, &curr_partner, dir, true);
                    return can_i_move && can_partner_move;
                }
            },
            &Cell::Robot => panic!("robot in box search"),
        }
    }

    fn move_boxes_rec(grid: &mut Grid<Cell>, curr: &BCoord, dir: &Direction, ignore_partner: bool) -> bool {
        // println!("Looking at {} {}", curr.y, curr.x);
        let next = curr.move_in(dir).unwrap();
        let v = grid.get(&next);
        // println!("Want to move to ({} {}) {:?}", next.y, next.x, v);
        match v {
            &Cell::Empty => {
                // println!("It's empty moving");
                move_to(grid, curr, &next);
                true
            },
            &Cell::Wall => {
                // println!("It's wall");
                false
            },
            &Cell::BoxL => {
                println!("Moving a boxL");
                if !can_move_boxes_rec(grid, &next, dir, ignore_partner) {
                    println!("It can't be moved");
                    return false
                }
                println!("It can be moved");
                if dir == &Direction::R || dir == &Direction::L {
                    println!("It's L/R");
                    let can_move = move_boxes_rec(grid, &next, dir, false);
                    if can_move {
                        // println!("Moving");
                        move_to(grid, curr, &next);
                    }
                    can_move
                } else if ignore_partner {
                    println!("My partner is being moved");
                    return move_boxes_rec(grid, &next, dir, false);
                } else {
                    println!("I'm being moved {} {}", curr.y, curr.x);
                    let can_i_move = move_boxes_rec(grid, &next, dir, false);
                    println!("I can move: {}", can_i_move);
                    let curr_partner = curr.move_in(&Direction::R).unwrap();
                    let next_partner = curr_partner.move_in(dir).unwrap();
                    println!("My partner movign to {} {}", next_partner.y, next_partner.x);
                    let can_partner_move = move_boxes_rec(grid, &next_partner, dir, true);
                    println!("My partner can move: {}", can_partner_move);

                    if can_i_move && can_partner_move {
                        println!("We moving");
                        move_to(grid, curr, &next);
                        move_to(grid, &curr_partner, &next_partner);
                        return true
                    }
                    return false
                }
            },
            &Cell::BoxR => {
                if !can_move_boxes_rec(grid, &next, dir, ignore_partner) {
                    return false
                }

                if dir == &Direction::R || dir == &Direction::L {
                    let can_move = move_boxes_rec(grid, &next, dir, false);
                    if can_move {
                        // println!("Moving");
                        move_to(grid, curr, &next);
                    }
                    can_move
                } else if ignore_partner {
                    return move_boxes_rec(grid, &next, dir, false);
                } else {
                    let can_i_move = move_boxes_rec(grid, &next, dir, false);
                    let curr_partner = curr.move_in(&Direction::L).unwrap();
                    let can_partner_move = move_boxes_rec(grid, &curr_partner, dir, true);

                    if can_i_move && can_partner_move {
                        move_to(grid, curr, &next);
                        move_to(grid, &curr_partner, &curr_partner.move_in(dir).unwrap());
                        return true
                    }
                    return false;
                }
            },
            &Cell::Robot => panic!("robot in box search"),
        }
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
                if move_boxes_rec(&mut grid, &current_position, &direction, false) {
                    current_position = want_to_move_to;
                }
            },
            Cell::BoxR => {
                if move_boxes_rec(&mut grid, &current_position, &direction, false) {
                    current_position = want_to_move_to;
                }
            },
            Cell::Robot => panic!("trying to move to robot"),
        };

        grid.print(|c| match c {
            Cell::Empty => '.',
            Cell::Wall => '#',
            Cell::Robot => '@',
            Cell::BoxL => '[',
            Cell::BoxR => ']',
        });
        println!()
    }


    grid.grid.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(acc, move |acc, (x, cell)| {
            if cell != &Cell::BoxL || cell != &Cell::BoxR { acc }
            else {
                acc + (y*100) + x
            }
        })
    });
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