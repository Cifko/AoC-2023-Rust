use std::cell;

use crate::common::get_lines;

pub fn solve() {
    println!("Day 6 of 2015");
    let lines = get_lines();
    let mut grid = Vec::new();
    let mut grid2 = Vec::new();
    for _ in 0..1000 {
        let mut row = Vec::new();
        let mut row2 = Vec::new();
        for _ in 0..1000 {
            row.push(false);
            row2.push(0u64);
        }
        grid.push(row);
        grid2.push(row2);
    }
    for line in &lines {
        let (_, [cmd, x0, y0, x1, y1]): (&str, [&str; 5]) =
            regex::Regex::new(r"(turn off|toggle|turn on) (\d+),(\d+) through (\d+),(\d+)")
                .unwrap()
                .captures(line)
                .unwrap()
                .extract();
        let x0 = x0.parse::<usize>().unwrap();
        let y0 = y0.parse::<usize>().unwrap();
        let x1 = x1.parse::<usize>().unwrap();
        let y1 = y1.parse::<usize>().unwrap();
        match cmd {
            "turn off" => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        grid[x][y] = false;
                        grid2[x][y] = grid2[x][y].saturating_sub(1);
                    }
                }
            }
            "toggle" => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        grid[x][y] = !grid[x][y];
                        grid2[x][y] += 2;
                    }
                }
            }
            "turn on" => {
                for x in x0..=x1 {
                    for y in y0..=y1 {
                        grid[x][y] = true;
                        grid2[x][y] += 1;
                    }
                }
            }
            _ => panic!("Unknown command"),
        };
    }
    let mut s1 = 0;
    let mut s2 = 0;
    for row in grid {
        for cell in row {
            if cell {
                s1 += 1;
            }
        }
    }
    for row in grid2 {
        for cell in row {
            s2 += cell;
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
