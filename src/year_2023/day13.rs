use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn test_mirror(grid: &Vec<Vec<char>>, col: usize) -> u64 {
    let mut i = 0;
    let mut diff = 0;
    while col - i > 0 && col + i < grid.len() {
        for j in 0..grid[0].len() {
            if grid[col - i - 1][j] != grid[col + i][j] {
                diff += 1;
            }
        }
        i += 1;
    }
    diff
}

fn transpose<T: Clone>(v: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let rows = v.len();
    let cols = v[0].len();
    let mut transposed = vec![vec![v[0][0].clone(); rows]; cols];
    for r in 0..rows {
        for c in 0..cols {
            transposed[c][r] = v[r][c].clone();
        }
    }
    transposed
}

fn solution(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut s1 = 0;
    let mut s2 = 0;
    let transposed = transpose(grid);
    for col in 0..grid.len() {
        match test_mirror(grid, col) {
            0 => s1 += col * 100,
            1 => s2 += col * 100,
            _ => (),
        }
    }
    for col in 0..transposed.len() {
        match test_mirror(&transposed, col) {
            0 => s1 += col,
            1 => s2 += col,
            _ => (),
        }
    }
    (s1, s2)
}

pub fn solve() {
    println!("Day 13 of 2023");
    let mut lines = get_lines();
    lines.push("".to_string());
    let mut s1 = 0;
    let mut s2 = 0;
    let mut grid = Vec::new();
    for line in &lines {
        if line.is_empty() {
            let (a, b) = solution(&grid);
            s1 += a;
            s2 += b;
            grid.clear();
            continue;
        } else {
            grid.push(line.chars().collect());
        }
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
