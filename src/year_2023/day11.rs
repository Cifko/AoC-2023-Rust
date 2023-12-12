use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

pub fn solve() {
    println!("Day 11 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut has_galaxy = HashSet::new();
    let mut grid = Grid::new();
    let mut galaxies = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                has_galaxy.insert(x);
                galaxies.insert((x, y));
            }
        }
        if !line.contains('#') {
            grid.add_row(vec!['e'; line.len()])
        } else {
            grid.add_row(line.chars().collect());
        }
    }
    for x in 0..grid.grid[0].len() {
        if !has_galaxy.contains(&x) {
            for y in 0..grid.grid.len() {
                grid.grid[y][x] = 'e';
            }
        }
    }
    for galaxy0 in &galaxies {
        for galaxy1 in &galaxies {
            if galaxy0 >= galaxy1 {
                continue;
            }
            let mut dist = 0;
            let mut empty = 0;
            for x in cmp::min(galaxy0.0, galaxy1.0)..cmp::max(galaxy0.0, galaxy1.0) {
                match grid.grid[0][x] {
                    'e' => empty += 1,
                    _ => dist += 1,
                }
            }
            for y in cmp::min(galaxy0.1, galaxy1.1)..cmp::max(galaxy0.1, galaxy1.1) {
                match grid.grid[y][0] {
                    'e' => empty += 1,
                    _ => dist += 1,
                }
            }
            s1 += dist + empty * 2;
            s2 += dist + empty * 1000000;
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
