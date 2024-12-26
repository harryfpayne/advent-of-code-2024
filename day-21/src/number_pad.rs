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

    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
 */
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    y: i8,
    x: i8,
}
impl Key {
    fn apply(&self, dir: &Key) -> Key {
        Key {
            y: self.y + dir.y,
            x: self.x + dir.x,
        }
    }
}

pub fn get_num_position(c: &char) -> Key {
    match c {
        '7' => Key { x: 0, y: 0 },
        '8' => Key { x: 1, y: 0 },
        '9' => Key { x: 2, y: 0 },
        '4' => Key { x: 0, y: 1 },
        '5' => Key { x: 1, y: 1 },
        '6' => Key { x: 2, y: 1 },
        '1' => Key { x: 0, y: 2 },
        '2' => Key { x: 1, y: 2 },
        '3' => Key { x: 2, y: 2 },
        '0' => Key { x: 1, y: 3 },
        'A' => Key { x: 2, y: 3 },
        _ => panic!("invalid key"),
    }
}

pub fn get_dir_position(c: &char) -> Key {
    match c {
        '^' => Key { x: 1, y: 0 },
        'A' => Key { x: 2, y: 0 },
        '<' => Key { x: 0, y: 1 },
        'v' => Key { x: 1, y: 1 },
        '>' => Key { x: 2, y: 1 },
        _ => panic!("invalid key"),
    }
}

fn get_dir_offset(c: &char) -> Key {
    match c {
        '^' => Key { x: 0, y: -1 },
        'v' => Key { x: 0, y: 1 },
        '<' => Key { x: -1, y: 0 },
        '>' => Key { x: 1, y: 0 },
        _ => panic!("invalid direction"),
    }
}

const NUM_DEAD_CELL: Key = Key{y: 3, x: 0};
const DIR_DEAD_CELL: Key = Key{y: 0, x: 0};

fn follow_path(
    start: &Key,
    dead_cell: &Key,
    da: i8,
    db: i8,
    a_dir_char: &char,
    b_dir_char: &char,
) -> Option<String> {
    let a_dir = get_dir_offset(a_dir_char);
    let b_dir = get_dir_offset(b_dir_char);

    let mut seq = String::new();
    let mut curr = start.clone();
    for a in 0..da.abs() {
        curr = curr.apply(&a_dir);
        seq.push(*a_dir_char);
        if &curr == dead_cell {
            return None
        }
    }

    for b in 0..db.abs() {
        curr = curr.apply(&b_dir);
        seq.push(*b_dir_char);
        if &curr == dead_cell {
            return None
        }
    }

    seq.push('A');

    Some(seq)
}

pub fn find_shortest_paths(start: &Key, end: &Key, num_mode: bool) -> Vec<String> {
    if start == end {
        return vec!["A".to_string()]
    }
    let dy = end.y - start.y;
    let dx = end.x - start.x;

    let y_dir_char = if dy > 0 { 'v' } else { '^' };
    let x_dir_char = if dx > 0 { '>' } else { '<' };
    let dead_cell = if num_mode {NUM_DEAD_CELL} else {DIR_DEAD_CELL};

    let path1 = if dy.abs() > 0 {
        follow_path(
            &start,
            &dead_cell,
            dy,
            dx,
            &y_dir_char,
            &x_dir_char
        )
    } else {
        None
    };
    let path2 = if dx.abs() > 0 {
        follow_path(
            &start,
            &dead_cell,
            dx,
            dy,
            &x_dir_char,
            &y_dir_char,
        )
    } else {
        None
    };

    vec![path1, path2].into_iter().filter_map(|x| x).collect()
}



