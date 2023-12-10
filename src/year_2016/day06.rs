use std::collections::{HashMap, HashSet};

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 6 of 2016");
    let lines = get_lines();
    let mut rep = vec![HashMap::new(); 8];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            *rep[i].entry(c).or_insert(0) += 1;
        }
    }
    let s1 = rep
        .iter()
        .map(|rep| rep.iter().max_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap())
        .collect::<String>();
    let s2 = rep
        .iter()
        .map(|rep| rep.iter().min_by_key(|&(_, v)| v).map(|(k, _)| k).unwrap())
        .collect::<String>();
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
