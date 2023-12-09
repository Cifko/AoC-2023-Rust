use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 2 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = String::new();
    let mut x = 0;
    let mut y = 0;
    for line in &lines {
        for c in line.chars() {
            match c {
                'U' => y = (y - 1).max(0),
                'D' => y = (y + 1).min(2),
                'L' => x = (x - 1).max(0),
                'R' => x = (x + 1).min(2),
                _ => panic!("Unknown direction"),
            }
        }
        s1 = s1 * 10 + y * 3 + x + 1;
    }
    println!("Part 1: {}", s1);
    let mut x = -2i32;
    let mut y = 0i32;
    for line in &lines {
        for c in line.chars() {
            let mut nx = x;
            let mut ny = y;
            match c {
                'U' => ny = y - 1,
                'D' => ny = y + 1,
                'L' => nx = x - 1,
                'R' => nx = x + 1,
                _ => panic!("Unknown direction"),
            }
            if nx.abs() + ny.abs() <= 2 {
                x = nx;
                y = ny;
            }
        }
        match (x, y) {
            (0, -2) => s2 += "1",
            (-1, -1) => s2 += "2",
            (0, -1) => s2 += "3",
            (1, -1) => s2 += "4",
            (-2, 0) => s2 += "5",
            (-1, 0) => s2 += "6",
            (0, 0) => s2 += "7",
            (1, 0) => s2 += "8",
            (2, 0) => s2 += "9",
            (-1, 1) => s2 += "A",
            (0, 1) => s2 += "B",
            (1, 1) => s2 += "C",
            (0, 2) => s2 += "D",
            _ => panic!("Unknown position"),
        }
    }
    println!("Part 2: {}", s2);
}
