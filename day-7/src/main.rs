use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{}", part_1(INPUT));
    println!("elapsed: {:?}", s.elapsed());

    let s = Instant::now();
    println!("{}", part_2(INPUT));
    println!("elapsed: {:?}", s.elapsed());

    let s = Instant::now();
    println!("{}", part_2_fast(INPUT));
    println!("elapsed: {:?}", s.elapsed());
}

fn part_2_fast(input: &str) -> i64 {
    type Operation = usize;
    const ADD: Operation = 0;
    const MUL: Operation = 1;
    const CAT: Operation = 2;

    let equations = parse(input);

    fn get_nearest_power_of_10(value: i64) -> i64 {
        if value == 1 {
            return 10
        }
        10i64.pow((value as f64).log10().ceil() as u32)
    }
    fn get_valid_operations(total: &i64, value: &i64) -> [bool; 3] {
        [
            total >= value,
            total % value == 0,
            &(total % get_nearest_power_of_10(*value)) == value,
        ]
    }

    fn recurse(total: &i64, values: Vec<i64>) -> bool {
        if values.len() == 0 && total == &0 {
            return true;
        }
        if values.len() == 0 || total == &0 {
            return false;
        }

        let value = values[0];
        let operations = get_valid_operations(&total, &values[0]);
        if operations[ADD] {
            if recurse(&(total - value), values[1..].to_vec()) {
                return true;
            }
        }

        if operations[MUL] {
            if recurse(&(total / value), values[1..].to_vec()) {
                return true;
            }
        }

        if operations[CAT] {
            if recurse(&((total - value) / get_nearest_power_of_10(value)), values[1..].to_vec()) {
                return true;
            }
        }

        false
    }

    let mut sum = 0;
    for (total, mut values) in equations {
        values.reverse();

        if recurse(&total, values) {
            sum += total;
        }
    }

    sum
}


fn part_1(input: &str) -> i64 {
    let equations = parse(input);

    fn compute(numbers: &Vec<i64>, operators: &Vec<i64>) -> i64 {
        // operators: 0 = +, 1 = *
        let mut count = numbers[0];
        for (number, operation) in numbers[1..].iter().zip(operators.iter()) {
            match operation {
                0 => count += number,
                1 => count *= number,
                _ => panic!("Invalid operation"),
            };
        }

        count
    }

    fn inc(arr: &mut Vec<i64>) {
        let mut carry = 1;
        for i in (0..arr.len()).rev() {
            match (arr[i], carry) {
                (_, 0) => {},
                (0, 1) => {
                    arr[i] = 1;
                    carry = 0;
                },
                (1, 1) => arr[i] = 0,
                _ => panic!("Invalid arr")
            };
        }
    }

    let mut sum = 0;
    'equation_loop: for (total, numbers) in equations {

        let mut operators = vec![0; numbers.len() - 1];

        for i in 0..2i64.pow((numbers.len() - 1) as u32) {
            let total_ = compute(&numbers, &operators);
            if total_ == total {
                sum += total;
                continue 'equation_loop
            }

            inc(&mut operators)
        }
    }



    sum
}

fn part_2(input: &str) -> i64 {
    let equations = parse(input);

    fn concat(a: &i64, b: &i64) -> i64 {
        (a.to_string() + &*b.to_string()).parse::<i64>().unwrap()
    }

    fn compute(numbers: &Vec<i64>, operators: &Vec<i64>) -> i64 {
        // operators: 0 = +, 1 = *, 2 = ||
        let mut count = numbers[0];
        for (number, operation) in numbers[1..].iter().zip(operators.iter()) {
            match operation {
                0 => count += number,
                1 => count *= number,
                2 => count = concat(&count, number),
                _ => panic!("Invalid operation"),
            };
        }

        count
    }

    fn inc(arr: &mut Vec<i64>) {
        let mut carry = 1;
        for i in (0..arr.len()).rev() {
            match (arr[i], carry) {
                (_, 0) => {},
                (0, 1) => {
                    arr[i] = 1;
                    carry = 0;
                },
                (1, 1) => {
                    arr[i] = 2;
                    carry = 0;
                },
                (2, 1) => arr[i] = 0,
                _ => panic!("Invalid arr")
            };
        }
    }

    let mut sum = Arc::new(Mutex::new(0));
    let mut handlers = Vec::new();
    for (total, numbers) in equations {
        let sum = Arc::clone(&sum);
        let handle = thread::spawn(move || {
            let mut operators = vec![0; numbers.len() - 1];
            for _ in 0..3i64.pow((numbers.len() - 1) as u32) {
                let total_ = compute(&numbers, &operators);
                if total_ == total {
                    *sum.lock().unwrap() += total_;
                    return
                }

                inc(&mut operators)
            }
        });

        handlers.push(handle);
    }

    for handle in handlers {
        handle.join().unwrap();
    }

    let x = sum.lock().unwrap().clone(); x
}

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    let mut out = vec![];

    for line in input.lines() {
        let (l, r) = line.split_once(": ").unwrap();
        let numbers = r.split(" ").map(|x| x.parse::<i64>().unwrap()).collect();

        out.push((l.parse::<i64>().unwrap(), numbers));
    }

    out
}