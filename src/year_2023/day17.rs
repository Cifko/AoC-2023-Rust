use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
};

use crate::helpers::{get_lines, parse, Grid};

fn flow(min_c: usize, max_c: usize, g: &Grid<u64>) -> u64 {
    let mut next = BinaryHeap::new();
    next.push(Reverse((0, 0, 0, 0, 0, 0)));
    let mut visited = HashSet::new();
    while !next.is_empty() {
        let (s, x, y, c, dx, dy) = next.pop().unwrap().0;
        let key = (x, y, c, dx, dy);
        if visited.contains(&key) {
            continue;
        }
        visited.insert(key);
        let mut s = s;
        if dx != 0 || dy != 0 {
            s += g[(x, y)];
        }
        if x == g.height() - 1 && y == g.width() - 1 && c >= min_c {
            return s;
        }
        for (nx, ny) in g.get4(x, y) {
            let ndx = nx as i64 - x as i64;
            let ndy = ny as i64 - y as i64;
            if ndx == -dx && ndy == -dy {
                continue;
            }
            let nc;
            if ndx == dx && ndy == dy {
                if c == max_c {
                    continue;
                }
                nc = c + 1;
            } else {
                if (dx != 0 || dy != 0) && c < min_c {
                    continue;
                }
                nc = 1;
            }
            next.push(Reverse((s, nx, ny, nc, ndx, ndy)));
        }
    }
    panic!("he?");
}

pub fn solve() {
    println!("Day 17 of 2023");
    let lines = get_lines();
    let mut s1 = 0u64;
    let mut s2 = 0u64;
    let mut grid = Grid::new();
    grid.add_rows(
        lines
            .into_iter()
            .map(|l| {
                l.chars()
                    .into_iter()
                    .map(|c| c as u64 - '0' as u64)
                    .collect()
            })
            .collect(),
    );
    s1 = flow(0, 3, &grid);
    s2 = flow(4, 10, &grid);
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
