/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
 */
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Pointer};
use *;
use crate::number_pad::Direction::*;

pub struct Key {
    y: i8,
    x: i8,
}

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum Direction {
    U, D, L, R, A
}

impl Debug for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::U => write!(f, "^"),
            Direction::D => write!(f, "v"),
            Direction::L => write!(f, "<"),
            Direction::R => write!(f, ">"),
            Direction::A => write!(f, "A"),
        }
    }
}


pub fn get_number_key(i: &i8) -> Key {
    match i {
        7 => Key{x: 0, y: 0},
        8 => Key{x: 1, y: 0},
        9 => Key{x: 2, y: 0},
        4 => Key{x: 0, y: 1},
        5 => Key{x: 1, y: 1},
        6 => Key{x: 2, y: 1},
        1 => Key{x: 0, y: 2},
        2 => Key{x: 1, y: 2},
        3 => Key{x: 2, y: 2},
        0 => Key{x: 1, y: 3},
        -1 => Key{x: 2, y: 3},
        _ => panic!("invalid key"),
    }
}

fn get_robot_key(d: &Direction) -> Key {
    match d {
        Direction::U => Key{y: 0, x: 1},
        Direction::A => Key{y: 0, x: 2},
        Direction::L => Key{y: 1, x: 0},
        Direction::D => Key{y: 1, x: 1},
        Direction::R => Key{y: 1, x: 2},
    }
}

pub fn get_number_sequence(start: &i8, end: &i8) -> Vec<Direction> {
    let s = get_number_key(start);
    let e = get_number_key(end);

    let x = e.x - s.x;
    let y = e.y - s.y;

    let mut moves = vec![];
    if y > 0 {
        moves.append(&mut vec![Direction::D; y as usize])
    } else if y < 0 {
        moves.append(&mut vec![Direction::U; (-y) as usize])
    }

    if x > 0 {
        moves.append(&mut vec![Direction::R; x as usize])
    } else if x < 0 {
        moves.append(&mut vec![Direction::L; (-x) as usize])
    }

    if s.x == 0 {
        moves.reverse()
    }

    moves.push(Direction::A);
    moves
}

pub fn get_robot_sequence(start: &Direction, end: &Direction) -> Vec<Direction> {
    let s = get_robot_key(start);
    let e = get_robot_key(end);

    let x = e.x - s.x;
    let y = e.y - s.y;

    let mut moves = vec![];
    if y > 0 {
        moves.append(&mut vec![Direction::D; y as usize])
    } else if y < 0 {
        moves.append(&mut vec![Direction::U; (-y) as usize])
    }

    if x > 0 {
        moves.append(&mut vec![Direction::R; x as usize])
    } else if x < 0 {
        moves.append(&mut vec![Direction::L; (-x) as usize])
    }

    if s.x == 0 {
        moves.reverse()
    }

    moves.push(Direction::A);
    moves
}