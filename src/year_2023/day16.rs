use std::{
    cmp,
    collections::{HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn get_beams(
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
    grid: &Grid<char>,
    cache: &mut HashMap<(i64, i64), HashSet<(i64, i64)>>,
) -> u64 {
    if x < 0 || y < 0 || x >= grid.width() as i64 || y >= grid.height() as i64 {
        cache.clear();
    }
    let x = x + dx;
    let y = y + dy;
    if x < 0 || y < 0 || x >= grid.width() as i64 || y >= grid.height() as i64 {
        return 0;
    }
    if cache.get(&(x, y)).map_or(false, |s| s.contains(&(dx, dy))) {
        return 0;
    }
    cache.entry((x, y)).or_default().insert((dx, dy));
    let mut r = (cache[&(x, y)].len() == 1) as u64;
    match grid[(x, y)] {
        '.' => r += get_beams(x, y, dx, dy, grid, cache),
        '/' => r += get_beams(x, y, -dy, -dx, grid, cache),
        '\\' => r += get_beams(x, y, dy, dx, grid, cache),
        '-' => {
            if dx == 0 {
                r += get_beams(x, y, 1, 0, grid, cache);
                r += get_beams(x, y, -1, 0, grid, cache);
            } else {
                r += get_beams(x, y, dx, dy, grid, cache);
            }
        }
        '|' => {
            if dy == 0 {
                r += get_beams(x, y, 0, 1, grid, cache);
                r += get_beams(x, y, 0, -1, grid, cache);
            } else {
                r += get_beams(x, y, dx, dy, grid, cache);
            }
        }
        _ => panic!("Invalid char"),
    }
    r
}

pub fn solve() {
    println!("Day 16 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut g = Grid::new();
    g.add_rows(lines.into_iter().map(|l| l.chars().collect()).collect());
    s1 = get_beams(-1, 0, 1, 0, &g, &mut HashMap::new());
    for x in 0..g.width() {
        s2 = s2.max(get_beams(x as i64, -1, 0, 1, &g, &mut HashMap::new()));
        s2 = s2.max(get_beams(
            x as i64,
            g.height() as i64,
            0,
            -1,
            &g,
            &mut HashMap::new(),
        ));
    }
    for y in 0..g.height() {
        s2 = s2.max(get_beams(-1, y as i64, 1, 0, &g, &mut HashMap::new()));
        s2 = s2.max(get_beams(
            g.width() as i64,
            y as i64,
            -1,
            0,
            &g,
            &mut HashMap::new(),
        ));
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
