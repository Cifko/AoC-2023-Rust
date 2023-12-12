use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn sol(
    line: &Vec<char>,
    index: usize,
    nums: Vec<usize>,
    force: bool,
    cache: &mut HashMap<(usize, Vec<usize>, bool), u64>,
) -> u64 {
    let key = (index, nums.clone(), force);
    if index == 0 {
        cache.clear();
    }
    if let Some(value) = cache.get(&key) {
        return *value;
    }
    let mut force = force;
    let mut index = index;
    let nums = if nums[0] == 0 {
        if nums.len() == 1 {
            for i in index..line.len() {
                if line[i] == '#' {
                    cache.insert(key, 0);
                    return 0;
                }
            }
            cache.insert(key, 1);
            return 1;
        }
        force = false;
        if line.len() == index {
            cache.insert(key, 0);
            return 0;
        }
        if line[index] == '#' {
            cache.insert(key, 0);
            return 0;
        }
        index += 1;
        nums[1..].to_vec()
    } else {
        nums
    };
    if !force {
        while line.len() > index && line[index] == '.' {
            index += 1;
        }
    }
    let nums_reduced = [[nums[0] - 1].to_vec(), nums[1..].to_vec()].concat();
    let mut s = 0;
    if line.len() > index {
        match line[index] {
            '?' => {
                s = sol(line, index + 1, nums_reduced, true, cache);
                if !force {
                    s += sol(line, index + 1, nums, false, cache)
                }
            }
            '#' => s = sol(line, index + 1, nums_reduced, true, cache),
            '.' => {
                if force {
                    s = 0;
                }
            }
            _ => panic!("Invalid char"),
        }
    }
    cache.insert(key, s);
    s
}

pub fn solve() {
    println!("Day 12 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut cache = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        let (line, nums) = line.split_once(' ').unwrap();
        let nums = nums
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        s1 += sol(&line.chars().collect(), 0, nums.clone(), false, &mut cache);
        s2 += sol(
            &[line; 5].join("?").chars().collect(),
            0,
            vec![nums.clone(); 5].concat(),
            false,
            &mut cache,
        );
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
