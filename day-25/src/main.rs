const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part_1(INPUT));
}

fn part_1(input: &str) {
    fn matches(lock: &[i32; 5], key: &[i32; 5]) -> bool {
        lock.iter().enumerate().all(|(i, v)| key[i] + v <= 5)
    }

    let (locks, keys) = parse(input);

    let mut count = 0;
    for lock in locks {
        for key in keys.iter() {
            if matches(&lock, key) {
                count += 1;
            }
        }
    }

    println!("{}", count)
}

fn parse(input: &str) -> (Vec<[i32; 5]>, Vec<[i32; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for pattern in input.split("\n\n") {
        let is_lock = pattern.starts_with("#");
        let mut pins = [-1; 5];

        for line in pattern.lines() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    pins[i] += 1
                }
            }
        }

        if is_lock {
            locks.push(pins);
        } else {
            keys.push(pins);
        }
    }

    (locks, keys)
}
