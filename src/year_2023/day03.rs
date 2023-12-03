use std::collections::HashMap;

use crate::common::get_lines;

pub fn solve() {
    println!("Day 03");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut is_number = false;
    let mut graph = HashMap::new();
    for (j, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if "0123456789".contains(c) {
                if !is_number {
                    let mut start = i;
                    while start > 0 && "0123456789".contains(line.chars().nth(start - 1).unwrap()) {
                        start -= 1;
                    }
                    let mut end = start;
                    while end < line.len() && "0123456789".contains(line.chars().nth(end).unwrap())
                    {
                        end += 1;
                    }
                    let num = &line[start..end];
                    let num = num.parse::<i32>().unwrap();
                    let mut symbol = false;
                    for x in start.saturating_sub(1)..=end {
                        for y in j.saturating_sub(1)..=(j + 1) {
                            if let Some(sl) = lines.get(y) {
                                if let Some(c) = sl.chars().nth(x) {
                                    if !"0123456789.".contains(c) {
                                        graph.entry((x, y)).or_insert(vec![]).push(num);
                                        symbol = true;
                                    }
                                }
                            }
                        }
                    }
                    if symbol {
                        s1 += num;
                    }
                }
                is_number = true;
            } else {
                is_number = false;
            }
        }
    }
    for ((x, y), v) in graph.iter() {
        if v.len() == 2 {
            if lines[*y].chars().nth(*x).unwrap() == '*' {
                s2 += v[0] * v[1];
            }
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
