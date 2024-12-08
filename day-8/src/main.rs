use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
}

fn part_1(input: &str) -> usize {
    let limit = input.lines().count() as i32;
    let antennas = parse(input);

    let mut points = HashSet::new();
    for (_, frequency) in antennas {
        for (i, p1) in frequency.iter().enumerate() {
            for p2 in frequency[i+1..].iter() {
                let diff = (p1.0 - p2.0, p1.1 - p2.1);
                let diff2 = (-diff.0, -diff.1);
                let mut an1 = (p1.0 + diff.0, p1.1 + diff.1);
                let mut an2 = (p2.0 + diff2.0, p2.1 + diff2.1);

                if !(an1.0 < 0 || an1.0 >= limit || an1.1 < 0 || an1.1 >= limit) {
                    points.insert(an1);
                }
                if !(an2.0 < 0 || an2.0 >= limit || an2.1 < 0 || an2.1 >= limit) {
                    points.insert(an2);
                }

            }
        }
    }

    points.len()
}

fn part_2(input: &str) -> usize {
    let limit = input.lines().count() as i32;
    let antennas = parse(input);

    let mut points = HashSet::new();
    for (_, frequency) in antennas {
        for (i, p1) in frequency.iter().enumerate() {
            for p2 in frequency[i+1..].iter() {
                let diff = (p1.0 - p2.0, p1.1 - p2.1);
                let diff2 = (-diff.0, -diff.1);
                let mut an1 = p1.clone();
                let mut an2 = p2.clone();

                while !(an1.0 < 0 || an1.0 >= limit || an1.1 < 0 || an1.1 >= limit) {
                    points.insert(an1);
                    an1 = (an1.0 + diff.0, an1.1 + diff.1)
                }
                while !(an2.0 < 0 || an2.0 >= limit || an2.1 < 0 || an2.1 >= limit) {
                    points.insert(an2);
                    an2 = (an2.0 + diff2.0, an2.1 + diff2.1)
                }
            }
        }
    }

    points.len()
}

fn parse(input: &str) -> HashMap<char, Vec<(i32, i32)>> {
    let mut output = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                output.entry(c).or_insert(vec![]).push((y as i32, x as i32));
            }
        }
    }

    output
}
