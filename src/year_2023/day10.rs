use std::collections::HashMap;

use crate::helpers::{get_lines, parse, Grid};

fn upscale(cell: &char) -> [[char; 3]; 3] {
    match cell {
        '.' => [[' ', ' ', ' '], [' ', '.', ' '], [' ', ' ', ' ']],
        'L' => [[' ', 'x', ' '], [' ', 'x', 'x'], [' ', ' ', ' ']],
        'F' => [[' ', ' ', ' '], [' ', 'x', 'x'], [' ', 'x', ' ']],
        '7' => [[' ', ' ', ' '], ['x', 'x', ' '], [' ', 'x', ' ']],
        'J' => [[' ', 'x', ' '], ['x', 'x', ' '], [' ', ' ', ' ']],
        '-' => [[' ', ' ', ' '], ['x', 'x', 'x'], [' ', ' ', ' ']],
        '|' => [[' ', 'x', ' '], [' ', 'x', ' '], [' ', 'x', ' ']],
        _ => panic!("Invalid cell {}", cell),
    }
}

pub fn solve() {
    println!("Day 10 of 2023");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut g = Grid::new();
    for line in &lines {
        g.add_row(line.chars().collect());
    }
    g.add_border('.');
    let (sx, sy) = g.find_one('S').expect("Where is start?");
    let left = "LF-".contains(g.grid[sy][sx - 1]);
    let right = "J7-".contains(g.grid[sy][sx + 1]);
    let down = "JL|".contains(g.grid[sy + 1][sx]);
    let up = "F7|".contains(g.grid[sy - 1][sx]);
    if left {
        if right {
            g.grid[sy][sx] = '-';
        } else if up {
            g.grid[sy][sx] = 'J';
        } else if down {
            g.grid[sy][sx] = '7';
        } else {
            panic!("Invalid start");
        }
    } else if right {
        if up {
            g.grid[sy][sx] = 'L';
        } else if down {
            g.grid[sy][sx] = 'F';
        } else {
            panic!("Invalid start");
        }
    } else if up {
        if down {
            g.grid[sy][sx] = '|';
        } else {
            panic!("Invalid start");
        }
    } else {
        panic!("Invalid start");
    }
    g.upscale3x3(upscale);
    let sx = sx * 3 + 1;
    let sy = sy * 3 + 1;
    let mut x = sx;
    let mut y = sy;
    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut path = Vec::new();
    let mut found = false;
    while !found {
        path.push((x, y));
        // println!("{} {}", x, y);
        for (nx, ny) in g.get4(x, y) {
            if nx == prev_x && ny == prev_y {
                continue;
            }
            if g.grid[ny][nx] == 'x' {
                prev_x = x;
                prev_y = y;
                x = nx;
                y = ny;
                if x == sx && y == sy {
                    found = true;
                }
                break;
            }
        }
    }
    for y in 0..g.grid.len() {
        for x in 0..g.grid[y].len() {
            if path.contains(&(x, y)) {
                g.grid[y][x] = 'x';
            } else {
                g.grid[y][x] = ' ';
                s2 += 1;
            }
        }
    }
    s2 -= g.flood_fill(0, 0, '.');
    s1 = path.len() / 6;
    s2 = (s2 - path.len() + 8) / 9;
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
