
const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{:?}", part_2(INPUT));
}

fn part_1(input: &str) -> i32 {
    let readings = parse(input);

    let mut safe_count = 0;
    for reading in readings {
        let rising = reading[0] < reading[1];
        let init = &reading[0];
        let sub = &reading[1..];

        let res = sub.iter().try_fold(init, |a, c| {
            let rising_check = if rising { a < c } else { a > c };
            let bounds_check = (a - c).abs() <= 3;
            if bounds_check && rising_check {
                return Ok(c)
            }
            return Err(c)
        });

        if res.is_ok() {
            safe_count += 1;
        }
    }


    safe_count
}

fn part_2(input: &str) -> i32 {
    fn get_first_error_index(reading: &Vec<i32>) -> Option<usize> {
        let rising = reading[0] < reading[1];
        let sub = &reading[1..];

        let mut prev = &reading[0];
        for (i, value) in sub.iter().enumerate() {
            let rising_check = if rising { prev < value } else { prev > value };
            let bounds_check = (prev - value).abs() <= 3;
            if bounds_check && rising_check {
                prev = value;
                continue;
            }
            return Some(i+1)
        }

        None
    }
    fn remove_index(reading: &Vec<i32>, index: usize) -> Vec<i32> {
        reading.iter().enumerate().filter(|(i, _)| *i != index).map(|(_, v)| v.clone()).collect()
    }

    let readings = parse(input);

    let mut safe_count = 0;

    'reading_loop: for reading in readings {
        if let Some(index) = get_first_error_index(&reading) {
            for i in 0..=3 { // Can't have neg usize so subtract 1 v
                if index + i < 2 {
                    continue
                }
                let sub = remove_index(&reading, index + i - 2);
                if get_first_error_index(&sub).is_none() {
                    safe_count += 1;
                    continue 'reading_loop;
                }
            }

        } else {
            safe_count += 1
        }
    }

    safe_count
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut out = Vec::new();
    for line in input.lines() {
        let parts = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap()).collect::<Vec<_>>();
        out.push(parts);
    }


    out
}
