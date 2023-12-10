use crate::helpers::{get_lines, parse};

pub fn solve() {
    println!("Day 8 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut v = vec![vec![false; 50]; 6];
    for line in &lines {
        let (parts, _) = parse(line);
        match parts[0] {
            "rect" => {
                let (a, b) = parts[1].split_once('x').unwrap();
                let a = a.parse::<usize>().unwrap();
                let b = b.parse::<usize>().unwrap();
                for i in 0..a {
                    for j in 0..b {
                        v[j][i] = true;
                    }
                }
            }
            "rotate" => match parts[1] {
                "row" => {
                    let y = parts[2][2..].parse::<usize>().unwrap();
                    let by = parts[4].parse::<usize>().unwrap();
                    for _ in 0..by {
                        let mut tmp = vec![false; 50];
                        for i in 0..50 {
                            tmp[(i + 1) % 50] = v[y][i];
                        }
                        v[y] = tmp;
                    }
                }
                "column" => {
                    let x = parts[2][2..].parse::<usize>().unwrap();
                    let by = parts[4].parse::<usize>().unwrap();
                    for _ in 0..by {
                        let mut tmp = vec![false; 6];
                        for i in 0..6 {
                            tmp[(i + 1) % 6] = v[i][x];
                        }
                        for i in 0..6 {
                            v[i][x] = tmp[i];
                        }
                    }
                }
                _ => panic!("Unknown instruction"),
            },
            _ => (),
        }
    }
    for l in &v {
        for &c in l {
            s1 += c as u64
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: ");
    for l in v {
        for c in l {
            print!("{}", if c { '#' } else { '.' });
        }
        println!();
    }
}
