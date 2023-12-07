use std::{cmp::Reverse, collections::HashMap};

use crate::helpers::get_lines;

fn race(time: u64, distance: u64) -> u64 {
    let mut s = 0;
    for t in 1..time {
        if (time - t) * t > distance {
            s += 1;
        }
    }
    s
}

fn hand_value(hand: &String) -> u64 {
    let mut cnt = HashMap::new();
    for c in hand.chars() {
        *cnt.entry(c).or_insert(0) += 1;
    }
    if cnt.values().any(|&v| v == 5) {
        return 7;
    }
    if cnt.values().any(|&v| v == 4) {
        return 6;
    }
    if cnt.values().any(|&v| v == 3) && cnt.values().any(|&v| v == 2) {
        return 5;
    }
    if cnt.values().any(|&v| v == 3) {
        return 4;
    }
    if cnt.values().filter(|&v| *v == 2).count() == 2 {
        return 3;
    }
    if cnt.values().any(|&v| v == 2) {
        return 2;
    }
    1
}

fn card_values(hand: &String) -> Vec<u64> {
    hand.chars()
        .map(|c| "23456789TJQKA".chars().position(|x| x == c).unwrap() as u64)
        .collect()
}

fn hand_value_j(hand: &String) -> u64 {
    if hand.contains("J") {
        let mut m = 0;
        for c in "23456789TQKA".chars() {
            let new_hand = hand.replace("J", c.to_string().as_str());
            m = std::cmp::max(m, hand_value(&new_hand));
        }
        m
    } else {
        hand_value(hand)
    }
}

fn card_values_j(hand: &String) -> Vec<u64> {
    hand.chars()
        .map(|c| "J23456789TQKA".chars().position(|x| x == c).unwrap() as u64)
        .collect()
}

pub fn solve() {
    println!("Day 07 of 2023");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut lines = lines
        .iter()
        .map(|x| x.split_once(' ').unwrap())
        .map(|(a, b)| (a.to_string(), b.parse::<u64>().unwrap()))
        .collect::<Vec<_>>();
    lines.sort_by_key(|(hand, _)| (hand_value(hand), card_values(hand)));
    for (i, (hand, value)) in lines.iter().enumerate() {
        s1 += value * (i + 1) as u64;
    }
    lines.sort_by_key(|(hand, _)| (hand_value_j(hand), card_values_j(hand)));
    for (i, (hand, value)) in lines.iter().enumerate() {
        s2 += value * (i + 1) as u64;
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
