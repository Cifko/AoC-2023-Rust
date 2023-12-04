use std::cmp;
use std::collections::HashMap;

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 02");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let (id, game) = line.split_once(':').unwrap();
        let mut maximal = HashMap::<&str, u64>::new();
        let mut possible_game = true;
        for row in game.trim().split(';') {
            for cubes in row.trim().split(",") {
                let (amount, color) = cubes.trim().split_once(' ').unwrap();
                let amount = amount.parse::<u64>().unwrap();
                match color {
                    "red" => possible_game &= amount <= 12,
                    "green" => possible_game &= amount <= 13,
                    "blue" => possible_game &= amount <= 14,
                    _ => panic!("Unknown color"),
                }
                maximal
                    .entry(color)
                    .and_modify(|m| {
                        *m = cmp::max::<u64>(*m, amount);
                    })
                    .or_insert(amount);
            }
        }
        s2 += maximal.values().fold(1, |mul, x| mul * x);
        if possible_game {
            let id = id.split_once(" ").unwrap().1.parse::<u64>().unwrap();
            s1 += id;
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
