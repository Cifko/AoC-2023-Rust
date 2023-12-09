use std::collections::HashMap;

use crate::helpers::get_lines;

fn compute(nums: &Vec<i64>) -> (i64, i64) {
    if nums.iter().all(|x| *x == 0) {
        return (0, 0);
    }
    let next_level = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(a, b)| b - a)
        .collect::<Vec<_>>();
    let (first, last) = compute(&next_level);
    (nums.first().unwrap() - first, last + nums.last().unwrap())
}

fn parse(input: &str) -> (Vec<&str>, Vec<i64>) {
    let parts = input.split_whitespace().collect::<Vec<_>>();
    let nums = parts.iter().filter_map(|x| x.parse::<i64>().ok()).collect();
    (parts, nums)
}

pub fn solve() {
    println!("Day 09 of 2023");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let (_, nums) = parse(line);
        let (first, last) = compute(&nums);
        s1 += last;
        s2 += first;
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
