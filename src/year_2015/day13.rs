use std::collections::{BTreeSet, HashMap};

use crate::helpers::{get_lines, Graph};

pub fn solve() {
    println!("Day 13 of 2015");
    let lines = get_lines();
    let mut g = Graph::new();
    let mut edges: HashMap<&str, HashMap<&str, i64>> = HashMap::new();
    for line in &lines {
        let re = regex::Regex::new(
            r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).",
        )
        .unwrap();
        let (_, [who, gain, happiness, next_to]) = re.captures(line.as_str()).unwrap().extract();
        let happiness = match gain {
            "gain" => -happiness.parse::<i64>().unwrap(),
            "lose" => happiness.parse::<i64>().unwrap(),
            _ => panic!(),
        };
        g.add_vertex(who, who);
        *edges
            .entry(who)
            .or_insert(HashMap::new())
            .entry(next_to)
            .or_default() += happiness;
        *edges
            .entry(next_to)
            .or_insert(HashMap::new())
            .entry(who)
            .or_default() += happiness;
        // g.add_directed_edge(who, next_to, happiness);
    }
    for (who, next_to) in edges {
        for (next_to, happiness) in next_to {
            if who < next_to {
                g.add_edge(who, next_to, happiness);
            }
        }
    }
    let x = g.djikstra(
        "Alice",
        0,
        BTreeSet::new(),
        |_, _, _| true,
        |u, backpack| !backpack.contains(u),
        |v| BTreeSet::from_iter([v.to_owned()]),
    );
    let mut m = 0;
    for (end, backpacks) in x {
        for (backpack, value) in backpacks {
            if backpack.len() == g.vertices.len() && end == "Alice" {
                m = m.min(value);
            }
        }
    }
    let s1 = -m;
    g.add_vertex("me", "me");
    let w = g.vertices.keys().map(|k| k.to_string()).collect::<Vec<_>>();
    for v in &w {
        g.add_edge("me", v.as_str(), 0);
    }
    let mut m = 0;
    let x = g.djikstra(
        "Alice",
        0,
        BTreeSet::new(),
        |_, _, _| true,
        |u, backpack| !backpack.contains(u),
        |v| BTreeSet::from_iter([v.to_owned()]),
    );
    for (end, backpacks) in x {
        for (backpack, value) in backpacks {
            if backpack.len() == g.vertices.len() && end == "Alice" {
                m = m.min(value);
            }
        }
    }
    let s2 = -m;
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
