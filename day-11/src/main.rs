use std::collections::{HashMap};
use std::time::Instant;

const INPUT: &str = "5178527 8525 22 376299 3 69312 0 275";

fn main() {
    let s = Instant::now();
    println!("{:?}", solver(INPUT, 25));
    println!("{:?}", solver(INPUT, 75));
    println!("{:?}", s.elapsed());
}

fn solver(input: &str, n: usize) -> usize {
    fn simulate(i: u64) -> (u64, Option<u64>) {
        if i == 0 {
            return (1, None);
        }
        let length = i.ilog10() + 1;
        if length % 2 == 0 {
            let power = 10u64.pow(length / 2);
            let l = i / power;
            let r = i % power;
            return (l, Some(r));
        }
        (i * 2024, None)
    }
    fn simulate_n(cache: &mut HashMap<(u64, usize), usize>, i: u64, n: usize) -> usize {
        if n == 0 {
            return 1;
        }

        if cache.contains_key(&(i,n)) {
            return cache.get(&(i,n)).unwrap().clone();
        }

        let res = simulate(i);
        if res.1.is_none() {
            let l = simulate_n(cache, res.0, n-1);
            cache.insert((i,n), l);
            return l;
        }
        let l = simulate_n(cache, res.0, n-1);
        let r = simulate_n(cache, res.1.unwrap(), n-1);

        cache.insert((i,n), l+r);
        l+r
    }

    let stones = parse(input);
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();

    let mut count = 0;
    for stone in stones {
        count += simulate_n(&mut cache, stone, n)
    }
    count
}

fn parse(input: &str) -> Vec<u64> {
    input.split(" ").map(|x| x.parse().unwrap()).collect()
}
