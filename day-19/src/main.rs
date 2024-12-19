use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("{:?}", s.elapsed())
}

fn part_2(input: &str) -> usize {
    fn rec(
        cache: &mut HashMap<usize, usize>,
        stripes: &Vec<Vec<char>>,
        pattern: &Vec<char>,
        i: usize
    ) -> usize {
        if let Some(v) = cache.get(&i) {
            return *v
        }

        if i == pattern.len() {
            return 1
        }

        let mut valid_i = vec![];
        'stripe_loop: for stripe in stripes {
            for (j, towel_s) in stripe.iter().enumerate() {
                if let Some(pattern_s) = pattern.get(i+j) {
                    if towel_s != pattern_s {
                        continue 'stripe_loop
                    }
                } else {
                    continue 'stripe_loop
                }
            }

            valid_i.push(i + stripe.len())
        }

        let mut count = 0;
        for i_next in valid_i {
            let o = rec(cache, stripes, pattern, i_next);
            cache.insert(i_next, o);
            count += o
        }
        count
    }

    let (stripes, patterns) = parse(input);

    let mut count = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        count += rec(&mut HashMap::new(), &stripes, pattern, 0);
    }
    count
}

fn part_1(input: &str) -> usize {
    fn rec(
        cache: &mut HashMap<usize, bool>,
        stripes: &Vec<Vec<char>>,
        pattern: &Vec<char>,
        i: usize
    ) -> bool {
        if let Some(v) = cache.get(&i) {
            return *v
        }

        if i == pattern.len() {
            return true
        }

        let mut valid_i = vec![];
        'stripe_loop: for stripe in stripes {
            for (j, towel_s) in stripe.iter().enumerate() {
                if let Some(pattern_s) = pattern.get(i+j) {
                    if towel_s != pattern_s {
                        continue 'stripe_loop
                    }
                } else {
                    continue 'stripe_loop
                }
            }

            valid_i.push(i + stripe.len())
        }

        for i_next in valid_i {
            let o = rec(cache, stripes, pattern, i_next);
            cache.insert(i_next, o);
            if o {
                return true
            }
        }
        false
    }

    let (stripes, patterns) = parse(input);

    let mut count = 0;
    for (i, pattern) in patterns.iter().enumerate() {
        if rec(&mut HashMap::new(), &stripes, pattern, 0) {
            count += 1
        }
    }
    count
}

fn parse(input: &str) -> (Vec<Vec<char>>, Vec<Vec<char>>) {
    let (t, b) = input.split_once("\n\n").unwrap();
    let s = t.split(", ").map(|p| p.chars().collect()).collect();
    let p = b.lines().map(|p| p.chars().collect()).collect();

    (s, p)
}
