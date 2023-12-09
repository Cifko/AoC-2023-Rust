use std::collections::HashMap;

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 08 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let rule = &lines[0];
    let re = regex::Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
    let mut rules = HashMap::new();
    for line in &lines[2..] {
        let (_, [pos, left, right]) = re.captures(line).unwrap().extract();
        rules.insert(pos, (left, right));
    }
    let mut pos = "AAA";
    while pos != "ZZZ" {
        for r in rule.chars() {
            match r {
                'L' => pos = &rules[pos].0,
                'R' => pos = &rules[pos].1,
                _ => panic!(),
            }
            s1 += 1;
        }
    }
    s2 = 1;
    for pos in rules.keys() {
        if pos.ends_with("A") {
            let mut pos = *pos;
            let mut i = 0u64;
            while !pos.ends_with("Z") {
                for r in rule.chars() {
                    match r {
                        'L' => pos = &rules[pos].0,
                        'R' => pos = &rules[pos].1,
                        _ => panic!(),
                    }
                    i += 1;
                }
            }
            s2 = num::integer::lcm(s2, i);
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
