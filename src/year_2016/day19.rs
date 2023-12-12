use std::collections::{BTreeSet, HashSet};

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 19 of 2016");
    let lines = get_lines();
    let elfs = lines[0].parse::<usize>().unwrap();
    let mut playing = BTreeSet::from_iter(0..elfs);
    let mut next = 0;
    for _ in 1..elfs {
        next = (next + 1) % elfs;
        while !playing.contains(&next) {
            next = (next + 1) % elfs;
        }
        playing.remove(&next);
        while !playing.contains(&next) {
            next = (next + 1) % elfs;
        }
    }
    let s1 = playing.iter().next().unwrap() + 1;
    println!("Part 1: {}", s1);

    let mut playing = BTreeSet::from_iter(0..elfs);
    let mut next_remove = elfs / 2;
    for _ in 1..elfs {
        playing.remove(&next_remove);
        while !playing.contains(&next_remove) {
            next_remove = (next_remove + 1) % elfs;
        }
        if playing.len() % 2 == 0 {
            next_remove = (next_remove + 1) % elfs;
            while !playing.contains(&next_remove) {
                next_remove = (next_remove + 1) % elfs;
            }
        }
    }
    let s2 = playing.iter().next().unwrap() + 1;
    println!("Part 2: {}", s2);
}
