use std::ops::BitAnd;
use std::thread;
use std::time::Instant;
use computer::*;
use crate::computer_fast::run;

mod computer;
mod computer_fast;

const INPUT: &str = include_str!("input.txt");

fn vec_are_eq(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a == b)
}

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("result: {:?}", s.elapsed());
}

fn part_2(input: &str) -> i64 {
    fn run_with(i: i64) -> Vec<i64> {
        let mut a = i;

        let mut o = vec![];
        while a != 0 {
            o.push(run(&mut a));
        }
        o
    }

    fn rec(
        seq: &Vec<i64>,
        matching_index: usize,
        i: i64,
    ) -> Option<i64> {
        let mut i = i;
        loop { // Crazy that rust doesn't have normal for loop syntax
            let o = run_with(i);
            // println!("{} {} {:?}", matching_index, i, o);

            if matching_index < seq.len() -1 {
                if o[matching_index + 1] != seq[matching_index + 1] {
                    // println!("I've exceeded the range");
                    return None;
                }
            }

            if o[matching_index] != seq[matching_index] {
                i += 8i64.pow(matching_index as u32);
            } else {
                // println!("Found match for {}", matching_index);
                if matching_index == 0 {
                    // println!("Found end");
                    return Some(i);
                }
                if let Some(j) = rec(seq, matching_index -1, i) {
                    return Some(j)
                }
                // println!("Didn't find sub match");
                i += 8i64.pow(matching_index as u32);
            }
        }
    }

    let computer = parse(input);
    let mut seq = computer.instructions.iter().map(|i| *i as i64).collect::<Vec<i64>>();
    let mut matching_index = seq.len() - 1;
    let i = 8i64.pow(matching_index as u32);
    rec(&seq, matching_index, i).unwrap()
}

fn part_1(input: &str) {
    'a: for i in 0..1 {
        let mut computer = parse(input);
        computer.registers[0] = 117440;

        let mut run_for = 0;
        let mut halted = false;
        while !halted {
            if !computer.step() {
                // println!("Halted");
                halted = true;
            }
            // println!("{:?}", computer);
            if !vec_are_eq(&computer.instructions, &computer.output) {
                // println!("Invalid out");
                break
            }
            if computer.instructions.len() == computer.output.len() {
                // println!("i works {}", i)
            }
            if run_for > 10_000_000 {
                // println!("Run for too long");
                break;
            }
            run_for += 1;
        }
    }
}

fn parse(input: &str) -> ComputerState {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut instructions = vec![];

    let (s, i) = input.split_once("\n\n").unwrap();
    let l = s.lines().collect::<Vec<&str>>();
    a = l[0].split_once(": ").unwrap().1.parse::<i64>().unwrap();
    b = l[1].split_once(": ").unwrap().1.parse::<i64>().unwrap();
    c = l[2].split_once(": ").unwrap().1.parse::<i64>().unwrap();
    instructions = i.split_once(": ").unwrap().1.split(",").map(|x| x.parse::<u8>().unwrap()).collect();

    ComputerState::new(
        [a,b,c],
        instructions,
    )
}
