use std::{collections::HashMap, hash::Hash};

use crate::helpers::{get_lines, parse};

#[derive(Copy, Clone)]
enum OutputType {
    Bot(i64),
    Output(i64),
}

pub fn solve() {
    println!("Day 10 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut bots: HashMap<i64, Vec<i64>> = HashMap::new();
    let mut rules = HashMap::new();
    let mut outputs: HashMap<i64, Vec<i64>> = HashMap::new();
    for line in &lines {
        let (parts, nums) = parse(line);
        match nums.len() {
            2 => {
                // input, e.g.: value 5 goes to bot 2
                bots.entry(nums[1]).or_default().push(nums[0]);
            }
            3 => {
                // bot rule e.g.: bot 2 gives low to bot 1 and high to bot 0
                let low = if parts[5] == "output" {
                    OutputType::Output(nums[1])
                } else {
                    OutputType::Bot(nums[1])
                };
                let high = if parts[10] == "output" {
                    OutputType::Output(nums[2])
                } else {
                    OutputType::Bot(nums[2])
                };
                rules.insert(nums[0], (low, high));
            }
            _ => panic!("Unexpected input: {}", line),
        }
    }
    let mut movement = true;
    while movement {
        movement = false;
        for (i, chips) in bots.clone() {
            if chips.len() == 2 {
                movement = true;
                let (low, high) = if chips[0] < chips[1] {
                    (chips[0], chips[1])
                } else {
                    (chips[1], chips[0])
                };
                if low == 17 && high == 61 {
                    s1 = i;
                }
                let (low_dest, high_dest) = rules[&i];
                match low_dest {
                    OutputType::Bot(b) => bots.entry(b).or_default().push(low),
                    OutputType::Output(o) => {
                        outputs.entry(o).or_default().push(low);
                    }
                }
                match high_dest {
                    OutputType::Bot(b) => bots.entry(b).or_default().push(high),
                    OutputType::Output(o) => {
                        outputs.entry(o).or_default().push(high);
                    }
                }
                bots.entry(i).and_modify(|chips| chips.clear()); //.clear();
            }
        }
    }
    s2 = outputs[&0][0] * outputs[&1][0] * outputs[&2][0];
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
