use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

pub fn solve() {
    println!("Day 18 of 2023");
    let lines = get_lines();
    let mut s1 = 0i64;
    let mut s2 = 0i64;
    let re = regex::Regex::new(r"([UDLR]) (\d+) .*").unwrap();
    let mut x = 0;
    let mut y = 0;
    for line in &lines {
        let (_, [d, r]) = re.captures(line).unwrap().extract();
        let r = r.parse::<i64>().unwrap();
        match d {
            "U" => {
                s1 -= r * x;
                y += r;
            }
            "D" => {
                s1 += r * x;
                y -= r;
            }
            "L" => {
                s1 -= r * y;
                x -= r;
            }
            "R" => {
                s1 += r * y;
                x += r;
            }
            _ => unreachable!(),
        }
        s1 += r;
    }
    s1 = s1 / 2 + 1;

    let mut x = 0;
    let mut y = 0;
    let re = regex::Regex::new(r".*#([\da-fA-F]+)(\d)\)").unwrap();
    for line in &lines {
        let (_, [r, d]) = re.captures(line).unwrap().extract();
        let r = i64::from_str_radix(r, 16).unwrap();
        match d {
            "3" => {
                s2 -= r * x;
                y += r;
            }
            "1" => {
                s2 += r * x;
                y -= r;
            }
            "2" => {
                s2 -= r * y;
                x -= r;
            }
            "0" => {
                s2 += r * y;
                x += r;
            }
            _ => unreachable!(),
        }
        s2 += r;
    }
    s2 = s2 / 2 + 1;

    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
