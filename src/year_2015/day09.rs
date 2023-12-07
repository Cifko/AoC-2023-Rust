use std::collections::BTreeSet;

use crate::helpers::{get_lines, Graph};

pub fn solve() {
    println!("Day 9 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let line_regex = regex::Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut g = Graph::<String, String>::new();
    // let mut cities = HashSet::new();
    for line in &lines {
        let (_, [from, to, distance]) = line_regex.captures(line).unwrap().extract();
        let from = from.to_string();
        let to = to.to_string();
        let distance = distance.parse::<usize>().unwrap();
        // edges.insert((from, to), distance);
        // edges.insert((to, from), distance);
        g.add_vertex(from.clone(), from.clone());
        g.add_vertex(to.clone(), to.clone());
        g.add_edge(from, to, distance as i64);
        // cities.insert(from);
        // cities.insert(to);
    }
    for start in g.vertices.keys() {
        let solution = g.djikstra(
            start.clone(),
            0,
            BTreeSet::from_iter([start.clone()]),
            |_, _, _| true,
            |u, backpack| !backpack.contains(u),
            |v| BTreeSet::from_iter([v.clone()]),
        );
        for (end, backpacks) in solution {
            for (backpack, value) in backpacks {
                if backpack.len() == g.vertices.len() {
                    if s1 == 0 || s1 > value {
                        s1 = value;
                    }
                }
            }
        }
    }
    g.mut_edges(|edge| -edge);
    for start in g.vertices.keys() {
        let solution = g.djikstra(
            start.clone(),
            0,
            BTreeSet::from_iter([start.clone()]),
            |_, _, _| true,
            |u, backpack| !backpack.contains(u),
            |v| BTreeSet::from_iter([v.clone()]),
        );
        for (end, backpacks) in solution {
            for (backpack, value) in backpacks {
                if backpack.len() == g.vertices.len() {
                    if s2 == 0 || s2 > value {
                        s2 = value;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", -s2);
}
