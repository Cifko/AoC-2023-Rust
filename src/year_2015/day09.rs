use std::collections::{BTreeSet, HashMap, HashSet};

use crate::helpers::{get_lines, Graph};

pub fn solve() {
    println!("Day 9 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let line_regex = regex::Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut g = Graph::<String, u8>::new();
    // let mut cities = HashSet::new();
    for line in &lines {
        let (_, [from, to, distance]) = line_regex.captures(line).unwrap().extract();
        let from = from.to_string();
        let to = to.to_string();
        let distance = distance.parse::<usize>().unwrap();
        // edges.insert((from, to), distance);
        // edges.insert((to, from), distance);
        g.add_vertex(from.clone(), 0);
        g.add_vertex(to.clone(), 0);
        g.add_edge(from, to, distance as u64);
        // cities.insert(from);
        // cities.insert(to);
    }
    for start in g.vertices.keys() {
        println!(
            "{:?}",
            g.djikstra(
                start.clone(),
                0,
                BTreeSet::new(),
                |_, _, _| { true },
                |_, _| true,
                |_| BTreeSet::new()
            )
        );
    }
}
