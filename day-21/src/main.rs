use number_pad::*;

mod number_pad;

const INPUT: &str = include_str!("input.txt");

fn main() {
    part_1(INPUT);
}

fn part_1(input: &str) {
    let sequences = parse(input);
    let nums = parse_num(input);
    const robot_depth: i32 = 2;

    let mut count = 0;
    for (i, sequence) in sequences.into_iter().enumerate() {
        let mut moves = get_number_sequence(&-1, &sequence[0]);
        for pair in sequence.windows(2) {
            let [curr, next] = pair else {todo!()};
            moves.append(&mut get_number_sequence(&curr, &next))
        }

        println!("{:?}", moves);

        for _ in 0..robot_depth {
            let mut moves_temp = get_robot_sequence(&Direction::A, &moves[0]);
            for pair in moves.windows(2) {
                let [curr, next] = pair else {todo!()};
                moves_temp.append(&mut get_robot_sequence(&curr, &next))
            }
            moves = moves_temp.clone();
            println!("{:?}", moves);
            moves_temp.clear();
        }
        println!("{:?}", moves);
        println!("{} {}", moves.len(), nums[i]);
        count +=  moves.len() as i32 * nums[i]
    }




    println!("{}", count);
}

fn parse(input: &str) -> Vec<Vec<i8>> {
    input.trim().lines().map(|l|
        l.chars().map(|c| match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            'A' => -1,
            _ => panic!(),
        }).collect()
    ).collect()
}

fn parse_num(input: &str) -> Vec<i32> {
    input.lines().map(|l| l.replace("A", "").parse().unwrap()).collect()
}
