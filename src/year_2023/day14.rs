use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn move_stone(grid: &mut Grid<char>, x: usize, y: usize, nx: i64, ny: i64) {
    if nx >= 0
        && ny >= 0
        && nx < grid.width() as i64
        && ny < grid.height() as i64
        && grid.grid[y][x] == 'O'
        && grid.grid[ny as usize][nx as usize] == '.'
    {
        grid.grid[ny as usize][nx as usize] = 'O';
        grid.grid[y][x] = '.';
    }
}

fn tilt(grid: &mut Grid<char>, dx: i64, dy: i64) {
    for _ in 0..cmp::max(grid.width(), grid.height()) * 2 {
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                move_stone(grid, x, y, x as i64 + dx, y as i64 + dy);
            }
        }
    }
}

fn score(grid: &Grid<char>) -> u64 {
    let mut s = 0u64;
    for y in 0..grid.height() {
        for x in 0..grid.width() {
            if grid.grid[y][x] == 'O' {
                s += (grid.height() - y) as u64;
            }
        }
    }
    s
}

pub fn solve() {
    println!("Day 14 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut g = Grid::new();
    g.add_rows(lines.iter().map(|line| line.chars().collect()).collect());
    tilt(&mut g, 0, -1);
    s1 = score(&g);
    let mut cache = HashMap::new();
    let mut scores = vec![0];
    for moves in 0..1000000000 {
        tilt(&mut g, -1, 0);
        tilt(&mut g, 0, 1);
        tilt(&mut g, 1, 0);
        scores.push(score(&g));
        if let Some(begin) = cache.insert(g.clone(), moves) {
            s2 = scores[begin + (1000000000 - begin) % (moves - begin)];
            break;
        }
        tilt(&mut g, 0, -1);
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
