use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 04 of 2023");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let re_nums = regex::Regex::new(r"\d+").unwrap();
    let mut won = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let (_, line) = line.split_once(':').unwrap();
        let (win, mine) = line.split_once('|').unwrap();
        let win: HashSet<u64> = HashSet::from_iter(
            re_nums
                .find_iter(win)
                .map(|x| x.as_str().parse::<u64>().unwrap()),
        );
        let mine: HashSet<u64> = HashSet::from_iter(
            re_nums
                .find_iter(mine)
                .map(|x| x.as_str().parse::<u64>().unwrap()),
        );
        let c = win.intersection(&mine).count() as u64;
        let mul = &*won.entry(i).or_default() + 1u64;
        s2 += mul;
        if c > 0 {
            s1 += 1 << (c - 1);
            for j in 1..=c {
                *won.entry(i + j as usize).or_insert(0) += mul;
            }
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
