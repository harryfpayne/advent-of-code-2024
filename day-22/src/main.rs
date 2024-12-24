use std::collections::{HashMap};
use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", part_1(INPUT).1);
    println!("{:?}", part_2(INPUT))
}

fn part_2(input: &str) -> i64 {
    let (history, _) = part_1(input);

    let sequence_values: Vec<HashMap<(i64, i64, i64, i64), i64>> = history.iter().map(|h|
        h.iter().rev()
        .tuple_windows() // Very useful function from itertools
        .map(|(a, b, c, d, e)| ((d - e, c - d, b - c, a - b), *a))
        .collect::<HashMap<_, _>>()
    ).collect();

    let mut total_sequence_values: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();
    for sequence_value_map in sequence_values {
        for (seq, v) in sequence_value_map {
            *total_sequence_values.entry(seq).or_default() += v;
        }
    }

    *total_sequence_values.values().max().unwrap()
}

fn part_1(input: &str) -> (Vec<Vec<i64>>, usize) {
    const R: usize = 2000;
    fn step(n: i64) -> i64 {
        let mut next = n;
        next ^= next << 6;
        next &= 16777216 -1;
        next ^= next >> 5;
        next &= 16777216 -1;
        next ^= next << 11;
        next &= 16777216 -1;
        next
    }

    let nums = parse(input);

    let mut out = vec![];
    out.push(nums);
    for i in 1..=R {
        let mut o = vec![];
        let p = out.get(i-1).unwrap();
        for num in p {
            o.push(step(*num))
        }
        out.push(o);
    }

    let a = out[R].iter().fold(0, |a,c| a+c) as usize;
    ((0..out[0].len())
         .map(|i| out.iter().map(|inner| inner[i].clone() % 10).collect::<Vec<_>>())
         .collect(), a)
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}