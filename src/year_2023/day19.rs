use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn apply(xmas: Vec<(u64, u64)>, rule: &str, rules: &HashMap<&str, &str>) -> Vec<Vec<(u64, u64)>> {
    // println!("rule {} xmas {:?}", rule, xmas);
    if xmas.iter().any(|(a, b)| a > b) {
        return vec![];
    }
    if !rule.contains(":") {
        match rule {
            "A" => return vec![xmas],
            "R" => return vec![],
            _ => return apply(xmas, rules[rule], rules),
        }
    }
    let (rule, rest) = rule.split_once(':').unwrap();
    let (trest, frest) = rest.split_once(",").unwrap();
    let i = "xmas".find(rule.chars().next().unwrap()).unwrap();
    let op = rule.chars().nth(1).unwrap();
    let v = rule[2..].parse::<u64>().unwrap();
    match op {
        '<' => {
            let mut txmas = xmas.clone();
            txmas[i].1 = xmas[i].1.min(v - 1);
            let mut r = apply(txmas, trest, rules);
            let mut fxmas = xmas.clone();
            fxmas[i].0 = xmas[i].0.max(v);
            r.extend(apply(fxmas, frest, rules));
            r
        }
        '>' => {
            let mut txmas = xmas.clone();
            txmas[i].0 = xmas[i].0.max(v + 1);
            let mut r = apply(txmas, trest, rules);
            let mut fxmas = xmas.clone();
            fxmas[i].1 = xmas[i].1.min(v);
            r.extend(apply(fxmas, frest, rules));
            r
        }
        _ => panic!("Unknown op {}", op),
    }
}

pub fn solve() {
    println!("Day 19 of 2023");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let re_rule = regex::Regex::new(r"(\w+)\{(.+)\}").unwrap();
    let re_xmas = regex::Regex::new(r"(\d+)").unwrap();
    let mut rules = HashMap::new();
    for (y, line) in lines.iter().enumerate() {
        if let Some(a) = re_rule.captures(line) {
            let (_, [a, b]) = a.extract();
            rules.insert(a, b);
        } else if !line.is_empty() {
            let xmas = re_xmas
                .captures_iter(line)
                .map(|x| x[0].parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            if !apply(xmas.iter().map(|&x| (x, x)).collect(), "in", &rules).is_empty() {
                s1 += xmas.iter().sum::<u64>();
            }
        }
    }

    let r = apply(vec![(1, 4000); 4], "in", &rules);
    for xmas in r {
        s2 += xmas.into_iter().fold(1, |acc, b| acc * (b.1 - b.0 + 1));
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
