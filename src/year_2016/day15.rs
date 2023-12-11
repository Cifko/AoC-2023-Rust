use crate::helpers::{get_lines, parse};

pub fn solve() {
    println!("Day 15 of 2016");
    let lines = get_lines();
    let re =
        regex::Regex::new(r"Disc #\d+ has (\d+) positions; at time=0, it is at position (\d+).")
            .unwrap();
    let mut s1 = 0;
    let mut rotation = 1;
    for (i, line) in lines.iter().enumerate() {
        let (_, [pos, start]) = re.captures(line).unwrap().extract();
        let pos: u64 = pos.parse().unwrap();
        let start: u64 = start.parse().unwrap();
        while (s1 + i as u64 + 1) % pos != (pos - start) % pos {
            s1 += rotation;
        }
        rotation *= pos;
    }
    println!("Part 1: {}", s1);
    let mut s2 = s1;
    let pos = 11;
    let start = 0;
    while (s2 + lines.len() as u64 + 1) % pos != (pos - start) % pos {
        s2 += rotation;
    }
    println!("Part 2: {}", s2);
}
