use number_pad::*;

mod number_pad;

const INPUT: &str = include_str!("input_test.txt");

fn main() {
    part_1(INPUT);
    part_2(INPUT);
}

fn tabs(count: i32) -> String {
    "\t".repeat(count as usize)
}

fn part_2(input: &str) {
    fn get_shortest_paths(seq: &String, depth: i32) -> Vec<String> {
        // println!("{} Getting shortest path for: {}", tabs(2-depth), seq);
        if depth == 0 {
            // println!("{} At base so returning", tabs(2-depth));
            return vec![seq.clone()]
        }

        let mut all_paths: Vec<String> = vec![];
        for pair in ("A".to_string() + seq.as_str()).chars().collect::<Vec<char>>().windows(2) {
            let [start, end] = pair else {panic!()};
            let s = get_dir_position(start);
            let e = get_dir_position(end);

            let paths = find_shortest_paths(&s, &e, false);
            // println!("{} Paths between {} and {} => {:?}",tabs(2-depth), start, end, paths);

            let mut shortest_len = usize::MAX;
            let mut shortest_paths = vec![];
            if paths.len() == 1 {
                shortest_paths = get_shortest_paths(&paths[0], depth -1);
            } else {
                for path in paths {
                    let mut shortest_paths_ = get_shortest_paths(&path, depth-1);
                    let shortest_len_ = shortest_paths_[0].len();

                    if shortest_len_ < shortest_len {
                        shortest_paths = shortest_paths_;
                        shortest_len = shortest_len_;
                    } else if shortest_len_ == shortest_len {
                        shortest_paths.append(&mut shortest_paths_);
                    }
                }
            }

            // println!("{} Shortest path between {} and {} => {:?}",tabs(2-depth), start, end, shortest_paths);
            if all_paths.len() == 0 {
                all_paths = shortest_paths;
                continue;
            }

            let mut next_all_paths = vec![];
            for path in all_paths.iter() {
                for new_path in shortest_paths.iter() {
                    let full_path = path.clone() + new_path.as_str();
                    next_all_paths.push(full_path)
                }
            }
            all_paths = next_all_paths;
        }

        // println!("{} All paths for {} => {:?}", tabs(2-depth), seq, all_paths);
        all_paths
    }

    fn get_all_paths(seq: &str, num_mode: bool) -> Vec<String> {
        // let seq = "A".to_string() + seq; TODO

        let mut all_paths: Vec<String> = vec![];
        for pair in seq.chars().collect::<Vec<char>>().windows(2) {
            let [start, end] = pair else {panic!()};
            let s;
            let e;
            if num_mode {
                s = get_num_position(start);
                e = get_num_position(end);
            } else {
                s = get_dir_position(start);
                e = get_dir_position(end);
            }

            let paths = find_shortest_paths(&s, &e, num_mode);

            if all_paths.len() == 0 {
                all_paths = paths;
                continue;
            }

            let mut next_all_paths = vec![];
            for path in all_paths.iter() {
                for new_path in paths.iter() {
                    let full_path = path.clone() + new_path.as_str();
                    next_all_paths.push(full_path)
                }
            }

            all_paths = next_all_paths;
        }

        all_paths
    }

    let sequences = parse(input);
    let nums = parse_num(input);
    let mut count = 0;
    for (sequence, num) in sequences.into_iter().zip(nums) {
        let num_paths = get_all_paths(&sequence, true);

        let mut paths = vec![];
        for path in num_paths {
            let path_ = get_shortest_paths(&path, 2);
            paths.push(path_);
        }

        let shortest = paths.iter().flatten().min_by(|a,b| a.len().cmp(&b.len())).unwrap();
        println!("{} {} {}", num, shortest, shortest.len())
    }
}

fn part_1(input: &str) {
    fn get_all_paths(seq: &str, num_mode: bool) -> Vec<String> {
        let seq = "A".to_string() + seq;

        let mut all_paths: Vec<String> = vec![];
        for pair in seq.chars().collect::<Vec<char>>().windows(2) {
            let [start, end] = pair else {panic!()};
            let s;
            let e;
            if num_mode {
                s = get_num_position(start);
                e = get_num_position(end);
            } else {
                s = get_dir_position(start);
                e = get_dir_position(end);
            }

            let paths = find_shortest_paths(&s, &e, num_mode);

            if all_paths.len() == 0 {
                all_paths = paths;
                continue;
            }

            let mut next_all_paths = vec![];
            for path in all_paths.iter() {
                for new_path in paths.iter() {
                    let full_path = path.clone() + new_path.as_str();
                    next_all_paths.push(full_path)
                }
            }

            all_paths = next_all_paths;
        }

        all_paths
    }

    let sequences = parse(input);
    let nums = parse_num(input);
    let mut count = 0;
    for (sequence, num) in sequences.into_iter().zip(nums) {
        let num_paths = get_all_paths(&sequence, true);

        let mut paths = num_paths;
        for i in 0..2 {
            let mut paths_ = vec![];
            for path in paths {
                let path_ = get_all_paths(path.as_str(), false);
                paths_.push(path_)
            }
            paths = paths_.into_iter().flatten().collect();
        }

        let shortest = paths.iter().min_by(|a,b| a.len().cmp(&b.len())).unwrap();
        count += num * shortest.len() as i32;
    }

    println!("{}", count);
}

fn parse(input: &str) -> Vec<String> {
    input.trim().lines().map(|l| l.to_string()).collect()
}

fn parse_num(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.replace("A", "").parse().unwrap()).collect()
}
