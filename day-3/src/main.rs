use regex::Regex;

const INPUT : &str = include_str!("input.txt");

fn main() {
    println!("{}", part_2(&INPUT));
}

fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

    let mut count = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        count += a.parse::<i32>().unwrap() * b.parse::<i32>().unwrap()
    }

    count
}

fn part_2(input: &str) -> i32 {
    let re = Regex::new(r"(mul\(([0-9]+),([0-9]+)\))|(do\(\))|(don\'t\(\))").unwrap();

    let mut count = 0;
    let mut mode_do = true;
    for capture in re.captures_iter(input) {
        let mut multiplier = 1;
        for group in capture.iter() {
            if group.is_none() {
                continue
            }
            let m = group.unwrap().as_str();
            match m {
                "don't()" => mode_do = false,
                "do()" => mode_do = true,
                _ => {
                    if let Ok(v) = m.parse::<i32>() {
                        multiplier *= v
                    }
                }
            }
        }

        if mode_do && multiplier > 1 {
            count += multiplier
        }
    }

    count
}


