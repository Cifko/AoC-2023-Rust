use std::{cmp::Reverse, collections::HashMap, hash::Hash};

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 4 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let (_, [room, sector, checksum]) = regex::Regex::new(r"(.*)-(\d+) ?\[(\w+)\]")
            .unwrap()
            .captures(line)
            .unwrap()
            .extract();
        let sector_id = sector.parse::<i64>().unwrap();
        let mut counts = HashMap::<char, i64>::new();
        for c in room.chars() {
            if c != '-' {
                *counts.entry(c).or_default() += 1;
            }
        }
        let mut counts: Vec<_> = counts
            .into_iter()
            .map(|(c, cnt)| (Reverse(cnt), c))
            .collect();
        counts.sort();
        let sum = counts[0..checksum.len()]
            .iter()
            .map(|(_, c)| c)
            .collect::<String>();
        let decrypted = room
            .chars()
            .into_iter()
            .map(|c| {
                if c == '-' {
                    ' '
                } else {
                    let c = c as u8 - b'a';
                    let c = (c + (sector_id % 26) as u8) % 26;
                    (c + b'a') as char
                }
            })
            .collect::<String>();
        if decrypted == "northpole object storage" {
            s2 = sector_id;
        }
        if sum == checksum {
            s1 += sector_id;
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
