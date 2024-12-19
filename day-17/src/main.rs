use std::ops::BitAnd;
use std::thread;
use std::time::Instant;
use computer::*;
use crate::computer_fast::BitMaskIterator;

mod computer;
mod computer_fast;

const INPUT: &str = include_str!("input_test.txt");

fn vec_are_eq(a: &Vec<u8>, b: &Vec<u8>) -> bool {
    a.iter().zip(b.iter()).all(|(a, b)| a == b)
}

fn main() {
    let s = Instant::now();
    println!("{:?}", part_2());
    println!("result: {:?}", s.elapsed());
}

fn part_2() -> usize {
    const TARGET: [i64; 16] = [2,4, 1,1, 7,5, 1,5, 0,3, 4,4, 5,5, 3,0];
    const N: i64 = 164_532_461_596_349;
    // const KNOWN_PATTERN: i64 = 0b101;
    // const KNOWN_PATTERN: i64 = 0b11011010110111_0_10111101;
    let mut one_mask = 0xFFFFFFFFFFFFF;
    let mut zero_mask = 0;


    // less than 164532461596349
    const I_THRESH: usize = 10;
    'a: for n in BitMaskIterator::new() {
        // let n = n_ << 3 | KNOWN_PATTERN;
        // println!("({:040b})", n);

        let mut i = 0;
        let mut a = n;
        let mut b = 0;
        let mut c = 0;

        while i < 16 {
            if i >= I_THRESH {
                one_mask = one_mask & n;
                zero_mask = zero_mask | n;
                println!("{}: {} \t\t ({:040b}) ({:040b}) ({:040b}) ", n, i, n, one_mask, zero_mask);
            }

            if a == 0 {
                continue 'a;
            }
            let o = computer_fast::run(&mut a, &mut b, &mut c);
            if o != TARGET[i] {
                continue 'a;
            } else {
                i += 1
            }
        }
        return n as usize;
    }

    0
}

fn part_1(input: &str) {
    'a: for i in 0..1 {
        if i % 10_000 == 0 {
            println!("{}", i);
        }

        let mut computer = parse(input);
        computer.registers[0] = 117440;

        let mut run_for = 0;
        let mut halted = false;
        while !halted {
            if !computer.step() {
                println!("Halted");
                halted = true;
            }
            println!("{:?}", computer);
            if !vec_are_eq(&computer.instructions, &computer.output) {
                println!("Invalid out");
                break
            }
            if computer.instructions.len() == computer.output.len() {
                println!("i works {}", i)
            }
            if run_for > 10_000_000 {
                println!("Run for too long");
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
