use crate::helpers::{get_lines, modulo_pow, parse};

pub fn solve() {
    println!("Day 25 of 2015");
    let lines = get_lines();
    let nums = regex::Regex::new(r"(\d+)")
        .unwrap()
        .find_iter(&lines[0])
        .map(|x| x.as_str())
        .collect::<Vec<_>>();
    let row = nums[0].parse::<usize>().unwrap();
    let col = nums[1].parse::<usize>().unwrap();
    let modulo = 33554393;
    let multiple = 252533;
    let start = 20151125;
    let pos = (1 + col) * col / 2 + col * (row - 1) + (row - 1) * (row - 2) / 2;
    let s1 = start * modulo_pow::<u64>(multiple, (pos - 1) as u64, modulo) % modulo;
    println!("Part 1: {}", s1);
}
