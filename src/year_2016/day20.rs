use num::PrimInt;

use crate::helpers::{get_lines, Intervals};

pub fn solve() {
    println!("Day 20 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut intervals = Intervals::new();
    for line in &lines {
        let mut parts = line.split('-');
        let start = parts.next().unwrap().parse::<u64>().unwrap();
        let end = parts.next().unwrap().parse::<u64>().unwrap();
        intervals.add_interval(start, end);
    }

    while let Some((a, b)) = intervals.get_covering_interval(s1) {
        s1 = b + 1;
    }
    let mut ip = 0;
    while ip < 4294967295 {
        while let Some((_, b)) = intervals.get_covering_interval(ip) {
            ip = b + 1;
        }
        ip += 1;
        if ip < 4294967295 {
            s2 += 1;
        }
    }

    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
