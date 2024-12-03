use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part_2(INPUT));
}

fn part_1(input: &str) -> i32 {
    let mut parsed = parse(INPUT);
    parsed[0].sort();
    parsed[1].sort();

    parsed[0].iter().zip(parsed[1].iter())
        .fold(0, |acc, (a, b)|  acc + (a - b).abs())
}

fn part_2(input: &str) -> i32 {
    let mut parsed = parse(INPUT);

    let mut freq_map = HashMap::new();

    parsed[1].iter().for_each(|&a| {
        *freq_map.entry(a).or_insert(0) += 1;
    });

    let mut sum = 0;
    parsed[0].iter().for_each(|&b| {
        if let Some(v) = freq_map.get(&b) {
            sum += b * v;
        }
    });

    sum
}

fn parse(input: &str) -> [Vec<i32>; 2] {
    let mut o: [Vec<i32>; 2] = [Vec::new(), Vec::new()];
    for line in input.trim().lines() {
        let chars = line.split("   ").collect::<Vec<&str>>();
        let l = chars[0].parse::<i32>().unwrap();
        let r = chars[1].parse::<i32>().unwrap();
        o[0].push(l);
        o[1].push(r);
    }
    o
}