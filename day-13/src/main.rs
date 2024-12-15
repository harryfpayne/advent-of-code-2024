use text_io::scan;

const INPUT: &str = include_str!("input.txt");

fn main() {
    println!("{}", part_1(INPUT));
    println!("{}", part_2(INPUT));
}

struct LinearEquation {
    // ax + by = c
    a: i64,
    b: i64,
    c: i64,
}

impl LinearEquation {
    fn new(a: i64, b: i64, c: i64) -> LinearEquation {
        LinearEquation { a, b, c }
    }
    fn intersection(&self, other: &LinearEquation) -> Option<(i64, i64)>{
        let a1 = self.a;
        let a2 = other.a;
        let b1 = self.b;
        let b2 = other.b;
        let c1 = self.c;
        let c2 = other.c;

        let det = a1 * b2 - a2 * b1;
        if det == 0 {
            return None
        }

        let x_numerator = c1 * b2 - c2 * b1;
        let y_numerator = a1 * c2 - a2 * c1;

        if x_numerator % det != 0 || y_numerator % det != 0 {
            return None
        }

        Some((x_numerator/det, y_numerator/det))
    }
}

fn part_1(input: &str) -> i64 {
    let machines = parse(input);
    let presses = machines.iter().map(|(a, b, p)| {
        let l = LinearEquation::new(a.0, b.0, p.0);
        let r = LinearEquation::new(a.1, b.1, p.1);
        l.intersection(&r)
    }).collect::<Vec<_>>();

    presses.into_iter().fold(0, |acc, press| match press {
        None => acc,
        Some((a,b)) => acc + (3*a as i64) + b as i64
    })
}

fn part_2(input: &str) -> i64 {
    let machines = parse(input);
    let presses = machines.iter().map(|(a, b, p)| {
        let l = LinearEquation::new(a.0, b.0, 10000000000000 + p.0);
        let r = LinearEquation::new(a.1, b.1, 10000000000000 + p.1);
        l.intersection(&r)
    }).collect::<Vec<_>>();

    presses.into_iter().fold(0, |acc, press| match press {
        None => acc,
        Some((a,b)) => acc + (3*a as i64) + b as i64
    })
}

fn parse(input: &str) -> Vec<((i64, i64), (i64, i64), (i64, i64))> {
    let mut out = Vec::new();

    for group in input.split("\n\n") {
        let line = group.split("\n").collect::<Vec<_>>();

        let mut ax: i64 = 0;
        let mut ay: i64 = 0;
        scan!(line[0].bytes() => "Button A: X+{}, Y+{}", ax, ay);

        let mut bx: i64 = 0;
        let mut by: i64 = 0;
        scan!(line[1].bytes() => "Button B: X+{}, Y+{}", bx, by);

        let mut x: i64 = 0;
        let mut y: i64 = 0;
        scan!(line[2].bytes() => "Prize: X={}, Y={}", x, y);

        out.push((
            (ax, ay),
            (bx, by),
            (x, y)
        ))
    }

    out
}
