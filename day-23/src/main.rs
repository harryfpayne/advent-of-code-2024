use std::collections::{HashMap, HashSet};
use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(&INPUT));
    println!("{:?}", part_2(&INPUT));
    println!("{:?}", s.elapsed())
}

fn part_2(input: &str) -> String {
    let connections = parse(input);
    let mut connection_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for connection in connections {
        connection_map.entry(connection.0).or_default().push(connection.1);
        connection_map.entry(connection.1).or_default().push(connection.0);
    }

    fn clique_size(
        map: &HashMap<&str, Vec<&str>>,
        clique: &mut HashSet<String>,
        curr: &str
    ){
        if !clique.iter().all(|c| map.get(curr).unwrap().contains(&c.as_str())) {
            return
        }
        clique.insert(curr.to_string());

        for next in map.get(curr).unwrap() {
            clique_size(map, clique, *next);
        }
    }

    let mut largest = HashSet::new();
    for node in connection_map.keys() {
        let mut a = HashSet::new();
        a.insert(node.to_string());
        for next in connection_map.get(node).unwrap() {
            clique_size(&connection_map, &mut a, *next);
        }

        if a.len() > largest.len() {
            largest = a;
        }
    }

    let mut l = largest.iter().map(|a| a.as_str()).collect::<Vec<_>>();
    l.sort();
    l.join(",")
}

fn part_1(input: &str) -> usize {
    let connections = parse(input);
    let mut connection_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for connection in connections {
        connection_map.entry(connection.0).or_default().push(connection.1);
        connection_map.entry(connection.1).or_default().push(connection.0);
    }

    fn dfs(
        map: &HashMap<&str, Vec<&str>>,
        visited: HashSet<&str>,
        origin: &str,
        k: &str,
        depth: usize
    ) -> Vec<Vec<String>> {
        if depth == 0 {
            if map.get(k).unwrap().contains(&origin) {
                return vec![visited.iter().map(|s| s.to_string()).collect()]
            }
            return vec![];
        }

        let mut paths = vec![];
        for conn in map.get(k).unwrap() {
            if visited.contains(*conn) {
                continue
            }

            let mut next_visited = visited.clone();
            next_visited.insert(*conn);
            let mut paths_ = dfs(map, next_visited, origin, *conn, depth -1);
            paths.append(&mut paths_)
        }

        paths
    }

    let mut loops: HashSet<Vec<String>> = HashSet::new();
    for k in connection_map.keys() {
        let o = dfs(&connection_map, HashSet::from_iter(vec![*k].into_iter()), *k, *k, 2);
        let s = o.into_iter().for_each(|mut l| {
            l.sort();
            loops.insert(l);
        });
    }

    let num = loops.iter().filter(|l| l.iter().any(|i| i.starts_with("t"))).count();

    num
}

fn parse(input: &str) -> Vec<(&str, &str)> {
    input.lines().map(|l| l.split_once("-").unwrap()).collect()
}
