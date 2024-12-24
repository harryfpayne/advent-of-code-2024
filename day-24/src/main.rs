use std::collections::{HashMap, HashSet};
use text_io::scan;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let mut a = Connection::new("a1".to_string(), "b1".to_string(), Gate::OR, "o1".to_string());
    let mut b = Connection::new("a2".to_string(), "b2".to_string(), Gate::OR, "o2".to_string());

    let mut l = vec![a, b];
    correct(&mut l, ("o1", "o2"));
    println!("{:?}", l);

    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
}

fn correct(connections: &mut Vec<Connection>, swap: (&str, &str)) {
    for connection in connections.iter_mut() {
        if connection.o == swap.0 {
            connection.change_output(swap.1);
        } else if connection.o == swap.1 {
            connection.change_output(swap.0);
        }
    }
}

fn check_for_zs(l: &HashSet<String>, exclude: Vec<&str>) -> Option<String> {
    for i in l.iter() {
        if i.starts_with("z") && !exclude.contains(&i.as_str()) {
            return Some(i.clone())
        }
    }
    None
}

fn part_2(input: &str) {
    let (_, mut connections) = parse(input.to_string());
    // Solved this by writing checks do describe an adder
    // when I find an error I swap them below and continue
    correct(&mut connections, ("nhs", "z20"));
    correct(&mut connections, ("wrc", "z34"));
    correct(&mut connections, ("nnf", "z09"));
    correct(&mut connections, ("ddn", "kqh"));


    let mut xor1_output = HashSet::new();
    let mut and1_output = HashSet::new();
    let mut carry_output = HashSet::new();
    for connection in connections.iter() {
        if (connection.a.starts_with("x") || connection.a.starts_with("y"))
            && (connection.b.starts_with("x") || connection.b.starts_with("y")) {
            if connection.g == Gate::XOR {
                xor1_output.insert(connection.o.clone());
            } else if connection.g == Gate::AND {
                and1_output.insert(connection.o.clone());
            } else {
                panic!("Invalid operation x|y {:?}", connection);
            }
        }
        if connection.g == Gate::OR {
            carry_output.insert(connection.o.clone());
        }
    }

    if let Some(z) = check_for_zs(&xor1_output, vec!["z00"]) { panic!("xor1 {}", z) }
    if let Some(z) = check_for_zs(&and1_output, vec![]) { panic!("and1 {}", z) }
    if let Some(z) = check_for_zs(&carry_output, vec!["z45"]) { panic!("carry {}", z) }

    carry_output.insert("dsr".to_string());

    let mut xor2_output = HashSet::new();
    let mut and2_output = HashSet::new();
    for xor1 in xor1_output.iter() {
        for connection in connections.iter() {
            if connection.a == *xor1 {
                if !carry_output.contains(&connection.b) {
                    panic!("xor1 connected to non carry {} {:?}", connection.a,  connection)
                }

                if connection.g == Gate::XOR {
                    xor2_output.insert(connection.o.clone());
                } else if connection.g == Gate::AND {
                    and2_output.insert(connection.o.clone());
                } else {
                    panic!("xor1 being or'd {:?}", connection)
                }
            } else if connection.b == *xor1 {
                if !carry_output.contains(&connection.a) {
                    panic!("xor1 connected to non carry {} {:?}", connection.b,  connection)
                }

                if connection.g == Gate::XOR {
                    xor2_output.insert(connection.o.clone());
                } else if connection.g == Gate::AND {
                    and2_output.insert(connection.o.clone());
                } else {
                    panic!("xor1 being or'd {:?}", connection)
                }
            }
        }
    }

    if let Some(z) = check_for_zs(&and2_output, vec![]) { panic!("and2 {}", z) }
}

fn part_1(input: &str) {
    let (mut values, mut connections) = parse(input.to_string());


    while connections.len() > 0 {
        let mut to_remove = vec![];
        for (i, conn) in connections.iter().enumerate() {
            if !values.contains_key(&conn.a) || !values.contains_key(&conn.b) {
                continue
            }

            let a = values.get(&conn.a).unwrap();
            let b = values.get(&conn.b).unwrap();
            let o = conn.apply(&a, &b);
            values.insert(conn.o.clone(), o);
            to_remove.push(i);
        }

        for (i, j) in to_remove.iter().enumerate() {
            connections.remove(j - i);
        }
    };

    let mut z = values.keys()
        .filter(|k| k.starts_with("z"))
        .collect::<Vec<_>>();
    z.sort();

    let mut o = 0i64;
    for i in 0..z.len() {
        if *values.get(z[i]).unwrap() {
            o += (1 << i)
        }
    }

    println!("{}", o);
}



#[derive(Debug, Eq, PartialEq, Clone)]
enum Gate {
    AND,
    OR,
    XOR
}

#[derive(Debug, Clone)]
struct Connection {
    a: String,
    b: String,
    g: Gate,
    o: String,
}
impl Connection {
    fn new(a: String, b: String, g: Gate, o: String) -> Self {
        Connection{a,b,g,o}
    }

    fn apply(&self, a: &bool, b: &bool) -> bool {
        match self.g {
            Gate::AND => a & b,
            Gate::OR => a | b,
            Gate::XOR => a ^ b,
        }
    }

    fn change_output(&mut self, o: &str) {
        self.o = o.to_string()
    }
}

fn parse(input: String) -> (HashMap<String, bool>, Vec<Connection>) {
    let (t, b) = input.split_once("\n\n").unwrap();

    let mut values = HashMap::new();
    for line in t.lines() {
        let (a, b) = line.split_once(": ").unwrap();
        let v = match b.parse::<i32>().unwrap() {
            0 => false,
            1 => true,
            _ => panic!(),
        };

        values.insert(a.to_string(), v);
    }


    let mut gates = vec![];
    for line in b.lines() {
        let mut a = String::new();
        let mut b = String::new();
        let mut g = String::new();
        let mut o = String::new();
        scan!(line.bytes() => "{} {} {} -> {}", a, g, b, o);

        let g = match g.as_str() {
            "AND" => Gate::AND,
            "OR" => Gate::OR,
            "XOR" => Gate::XOR,
            _ => panic!()
        };

        gates.push(Connection::new(a,b,g,o))
    }

    (values, gates)
}
