use std::collections::HashMap;

use crate::common::get_lines;

pub enum OP {
    AND(String, String),
    OR(String, String),
    LSHIFT(String, String),
    RSHIFT(String, String),
    NOT(String),
    CP(String),
}

pub fn get(s: &str, cmds: &HashMap<&str, OP>, values: &mut HashMap<String, u16>) -> u16 {
    if let Ok(x) = s.parse::<u16>() {
        return x;
    }
    if let Some(x) = values.get(s) {
        return *x;
    }
    let op = cmds.get(s).unwrap();
    let x = match op {
        OP::AND(left, right) => get(left, cmds, values) & get(right, cmds, values),
        OP::OR(left, right) => get(left, cmds, values) | get(right, cmds, values),
        OP::LSHIFT(left, right) => get(left, cmds, values) << get(right, cmds, values),
        OP::RSHIFT(left, right) => get(left, cmds, values) >> get(right, cmds, values),
        OP::NOT(left) => !get(left, cmds, values),
        OP::CP(left) => get(left, cmds, values),
    };
    values.insert(s.to_string(), x);
    x
}

pub fn solve() {
    println!("Day 7 of 2015");
    let lines = get_lines();
    let mut cmds = HashMap::new();
    let mut values = HashMap::new();
    for line in &lines {
        // let (_, [cmd, left, op, right, out]) =
        if let Some(x) =
            regex::Regex::new(r"([a-z0-9]*) (OR|RSHIFT|AND|LSHIFT) ([a-z0-9]*) -> (.*)")
                .unwrap()
                .captures(line)
        {
            let (_, [left, op, right, out]) = x.extract();
            match op {
                "OR" => cmds.insert(out, OP::OR(left.to_string(), right.to_string())),
                "RSHIFT" => cmds.insert(out, OP::RSHIFT(left.to_string(), right.to_string())),
                "AND" => cmds.insert(out, OP::AND(left.to_string(), right.to_string())),
                "LSHIFT" => cmds.insert(out, OP::LSHIFT(left.to_string(), right.to_string())),
                _ => panic!("Unrecognized op: {}", op),
            };
        } else if let Some(x) = regex::Regex::new(r"NOT ([a-z0-9]*) -> (.*)")
            .unwrap()
            .captures(line)
        {
            let (_, [left, out]) = x.extract();
            cmds.insert(out, OP::NOT(left.to_string()));
        } else if let Some(x) = regex::Regex::new(r"([a-z0-9]*) -> (.*)")
            .unwrap()
            .captures(line)
        {
            let (_, [left, out]) = x.extract();
            cmds.insert(out, OP::CP(left.to_string()));
        } else {
            panic!("Unrecognized line: {}", line);
        }
    }
    let s1 = get("a", &cmds, &mut values);
    values.clear();
    values.insert("b".to_string(), s1);
    let s2 = get("a", &cmds, &mut values);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
