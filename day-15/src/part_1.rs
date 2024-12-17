use crate::grid::grid::{BCoord, Direction, Grid};

const INPUT: &str = include_str!("input.txt");

pub fn part_1(input: &str) -> usize {
    let (mut grid, directions) = parse(input);

    fn move_to(grid: &mut Grid<Cell>, curr: &BCoord, next: &BCoord) {
        grid.set(next, *grid.get(&curr));
        grid.set(curr, Cell::Empty);
    }

    fn move_boxes_rec(grid: &mut Grid<Cell>, curr: &BCoord, dir: &Direction) -> bool {
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
            &Cell::Box => {
                // println!("It's box");
                let can_move = move_boxes_rec(grid, &next, dir);
                if can_move {
                    // println!("Moving");
                    move_to(grid, curr, &next);
                }
                can_move
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
            Cell::Box => {
                if move_boxes_rec(&mut grid, &current_position, &direction) {
                    current_position = want_to_move_to;
                }
            },
            Cell::Robot => panic!("trying to move to robot"),
        };

        // grid.print(|c| match c {
        //     Cell::Empty => '.',
        //     Cell::Wall => '#',
        //     Cell::Robot => '@',
        //     Cell::Box => 'O',
        // });
        // println!()
    }


    grid.grid.iter().enumerate().fold(0, |acc, (y, row)| {
        row.iter().enumerate().fold(acc, move |acc, (x, cell)| {
            if cell != &Cell::Box { acc }
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
    Box
}

fn parse(input: &str) -> (Grid<Cell>, Vec<Direction>) {
    let (g, d) = input.split_once("\n\n").unwrap();

    let g = g.lines().map(|l|
        l.chars().map(|c| match c {
            '.' => Cell::Empty,
            '#' => Cell::Wall,
            '@' => Cell::Robot,
            'O' => Cell::Box,
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
