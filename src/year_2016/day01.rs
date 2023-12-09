use std::collections::HashSet;

use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 1 of 2016");
    let lines = get_lines();
    let moves = lines[0].split(", ").collect::<Vec<_>>();
    let mut x = 0i32;
    let mut y = 0i32;
    let mut dx = 1;
    let mut dy = 0;
    let mut s2: Option<i32> = None;
    let mut visited = HashSet::new();
    for mov in moves {
        let (dir, dist) = mov.split_at(1);
        let dist = dist.parse::<i32>().unwrap();
        match dir {
            "L" => (dx, dy) = (dy, -dx),
            "R" => (dx, dy) = (-dy, dx),
            _ => panic!("Unknown direction"),
        }
        for _ in 0..dist {
            x += dx;
            y += dy;
            if s2.is_none() && !visited.insert((x, y)) {
                s2 = Some(x.abs() + y.abs());
            }
        }
    }
    println!("Part 1: {}", x.abs() + y.abs());
    println!("Part 2: {}", s2.unwrap());
}
