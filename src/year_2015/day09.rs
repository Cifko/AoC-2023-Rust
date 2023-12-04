use std::collections::{HashMap, HashSet};

use crate::helpers::{get_lines, Graph};

pub fn solve() {
    println!("Day 9 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut edges = HashMap::new();
    let mut cities = HashSet::new();
    let line_regex = regex::Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut g = Graph::<str, u8>::new();
    for line in &lines {
        let (_, [from, to, distance]) = line_regex.captures(line).unwrap().extract();
        let distance = distance.parse::<usize>().unwrap();
        g.add_vertex(from, None);
        g.add_vertex(to, None);
        edges.insert((from, to), distance);
        edges.insert((to, from), distance);
        cities.insert(from);
        cities.insert(to);
    }
    println!("{:?}", g);

    // let mut nothing_new = false;
    // while nothing_new {
    //     nothing_new = false;
    //     let mut new_combinations = Vec::new();
    //     for (visited, last, distance) in combinations {
    //         for city in &cities {
    //             if !visited.contains(&city.to_string()) {
    //                 let new_distance = distance + edges.get(&(last.clone(), city.clone())).unwrap();
    //                 new_combinations.push((visited.clone(), city.clone(), new_distance));
    //             }
    //         }
    //     }
    // }
    // println!("combos: {:?}", combinations);
    // println!("Part 1: {}", s1);
    // println!("Part 2: {}", s2);
}
