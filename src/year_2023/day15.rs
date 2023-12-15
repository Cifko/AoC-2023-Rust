use std::{
    cmp,
    collections::{HashMap, HashSet},
    ops::Index,
};

use crate::helpers::{get_lines, parse, Grid};

pub fn solve() {
    println!("Day 15 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    for (y, line) in lines.iter().enumerate() {
        let mut boxes = HashMap::new();
        let mut box_index = HashMap::new();
        let mut lenses = HashMap::new();
        for p in line.split(',') {
            let mut s = 0;
            for (i, c) in p.chars().enumerate() {
                if c == '=' {
                    let lens = p[i + 1..].parse::<u64>().unwrap();
                    let q = p[..i].to_string();
                    lenses.insert(q.clone(), lens);
                    if !box_index.contains_key(&q) {
                        boxes.entry(s).or_insert(Vec::new()).push(q.clone());
                        box_index.insert(q, s);
                    }
                } else if c == '-' {
                    let q = p[..i].to_string();
                    if box_index.contains_key(&q) {
                        let v = boxes.get_mut(&box_index[&q]).unwrap();
                        v.remove(v.iter().position(|x| x == &q).unwrap());
                        box_index.remove(&q);
                    }
                }
                s = (s + c as u64) * 17 % 256;
            }
            s1 += s;
        }
        for (box_num, boxes) in boxes {
            for (i, b) in boxes.iter().enumerate() {
                let lens = lenses[b];
                s2 += (box_num + 1) * (i as u64 + 1) * lens;
            }
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
