use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("input.txt");

fn main() {
    let parsed = parse(INPUT);
    println!("{:?} ", part_2(parsed.0, parsed.1));
}

fn order_meets_rules(rules: &Vec<(i32, i32)>, order: &Vec<i32>) -> bool {
    let mut order_indices: HashMap<&i32, usize> = HashMap::new();
    for (i, page) in order.iter().enumerate() {
        order_indices.insert(page, i);
    }

    for (l, r) in rules {
        if let Some(li) = order_indices.get(l) {
            if let Some(ri) = order_indices.get(r) {
                if li > ri {
                    return false;
                }
            }
        }
    }

    true
}

fn part_1(rules: Vec<(i32, i32)>, orders: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    for order in orders {
        if order_meets_rules(&rules, &order) {
            let mid = order[(order.len()) / 2];
            println!("{:?} is valid, adding {}", order, mid);
            count += mid
        }
    }

    count
}

fn part_2(rules: Vec<(i32, i32)>, orders: Vec<Vec<i32>>) -> i32 {
    // page: ([pages before], [pages after])
    let mut rules_map: HashMap<&i32, (HashSet<&i32>, HashSet<&i32>)> = HashMap::new();
    for (l, r) in rules.iter() {
        if !rules_map.contains_key(&l) {
            rules_map.insert(l, (HashSet::new(), HashSet::new()));
        }
        if !rules_map.contains_key(&r) {
            rules_map.insert(r, (HashSet::new(), HashSet::new()));
        }

        rules_map.get_mut(&l).unwrap().1.insert(r);
        rules_map.get_mut(&r).unwrap().0.insert(l);
    }

    let mut count = 0;
    for mut order in orders {
        if order_meets_rules(&rules, &order) {
            continue
        }

        order.sort_by(|a, b| {
            if let Some(rule) = rules_map.get(b) {
                if rule.1.contains(a) {
                    return std::cmp::Ordering::Greater;
                }
            }
            return std::cmp::Ordering::Less;
        });

        let mid = order[(order.len()) / 2];
        count += mid;
    }

    count
}

fn parse(input: &str) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let (rules_str, orders_str) = input.split_once("\n\n").unwrap();

    let mut rules = vec![];
    for rule in rules_str.lines() {
        let (l, r) = rule.split_once("|").unwrap();
        rules.push((l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()));
    }

    let mut orders = vec![];
    for order_str in orders_str.lines() {
        let mut order = vec![];
        for page in order_str.split(",") {
            order.push(page.parse::<i32>().unwrap());
        }

        orders.push(order);
    }


    (rules, orders)
}