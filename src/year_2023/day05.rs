use std::{
    collections::{btree_map::Range, HashMap, HashSet},
    hash::Hash,
};

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 03");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();
    let re_rules = regex::Regex::new(r"(\w+)-to-(\w+)").unwrap();
    let mut seeds = None;
    let mut rules = HashMap::new();
    let mut rules2 = HashMap::new();
    let mut rule = ("", "");
    for line in lines.iter() {
        if line.len() == 0 {
            continue;
        }
        if seeds.is_none() {
            seeds = Some(
                re_nums
                    .find_iter(line)
                    .map(|x| x.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            );
        } else {
            if line.contains(":") {
                let (_, [from, to]) = re_rules.captures(line).unwrap().extract();
                rules.insert(from, vec![]);
                rules2.insert(to, vec![]);
                rule = (from, to);
            } else {
                let nums = re_nums
                    .find_iter(line)
                    .map(|x| x.as_str().parse::<u64>().unwrap())
                    .collect::<Vec<_>>();
                rules
                    .get_mut(&rule.0)
                    .unwrap()
                    .push((rule.1, nums[0], nums[1], nums[2]));
                rules2
                    .get_mut(&rule.1)
                    .unwrap()
                    .push((rule.0, nums[0], nums[1], nums[2]));
            }
        }
    }
    let mut location = 0;
    let seeds = seeds.unwrap();
    let seeds2 = seeds
        .chunks(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect::<Vec<_>>();
    let mut min = None;
    for seed in seeds {
        let mut rule = "seed";
        let mut location = seed;
        while rule != "location" {
            for (next, a, b, c) in rules.get(&rule).unwrap() {
                rule = *next;
                if location >= *b && location < *b + *c {
                    location = location + *a - *b;
                    break;
                }
            }
        }
        if min.is_none() || location < min.unwrap() {
            min = Some(location);
        }
    }
    s1 = min.unwrap();
    loop {
        let mut seed = location;
        let mut rule = "location";
        while rule != "seed" {
            for (next, a, b, c) in rules2.get(&rule).unwrap() {
                rule = *next;
                if seed >= *a && seed < *a + *c {
                    seed = seed + *b - *a;
                    break;
                }
            }
        }

        let mut found = false;
        for (start, range) in &seeds2 {
            if seed >= *start && seed < *start + *range {
                s2 = location;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
        location += 1;
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
