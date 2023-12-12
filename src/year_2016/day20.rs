use num::PrimInt;

use crate::helpers::get_lines;

struct Intervals<T: PrimInt> {
    pub intervals: Vec<(T, T)>,
}

impl<T: PrimInt> Intervals<T> {
    pub fn new() -> Self {
        Self {
            intervals: Vec::new(),
        }
    }

    pub fn add_interval(&mut self, start: T, end: T) {
        let mut new_intervals = Vec::new();
        let mut start = start;
        let mut end = end;
        for (i_start, i_end) in self.intervals.iter() {
            if start > *i_end {
                new_intervals.push((*i_start, *i_end));
            } else if end < *i_start {
                new_intervals.push((start, end));
                start = *i_start;
                end = *i_end;
            } else {
                start = start.min(*i_start);
                end = end.max(*i_end);
            }
        }
        new_intervals.push((start, end));
        self.intervals = new_intervals;
    }

    pub fn is_covered(&self, value: T) -> bool {
        for (start, end) in &self.intervals {
            if start > &value {
                return false;
            }
            if end >= &value {
                return true;
            }
        }
        false
    }

    pub fn get_covering_interval(&self, value: T) -> Option<(T, T)> {
        for (start, end) in &self.intervals {
            if start > &value {
                return None;
            }
            if end >= &value {
                return Some((*start, *end));
            }
        }
        None
    }
}

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
        while let Some((a, b)) = intervals.get_covering_interval(ip) {
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
