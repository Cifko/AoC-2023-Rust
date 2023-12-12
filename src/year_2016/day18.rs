use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 18 of 2016");
    let lines = get_lines();
    let mut row = lines[0].chars().collect::<Vec<char>>();
    let width = row.len();
    let mut s1 = 0;
    let mut s2 = 0;
    for _ in 0..40 {
        let mut next_row = Vec::new();
        for (i, c) in row.iter().enumerate() {
            s1 += (*c == '.') as u64;
            let left_trap = if i == 0 { false } else { row[i - 1] == '^' };
            let right_trap = if i == width - 1 {
                false
            } else {
                row[i + 1] == '^'
            };
            if left_trap ^ right_trap {
                next_row.push('^');
            } else {
                next_row.push('.');
            }
        }
        row = next_row;
    }
    s2 = s1;
    for _ in 0..(400000 - 40) {
        let mut next_row = Vec::new();
        for (i, c) in row.iter().enumerate() {
            s2 += (*c == '.') as u64;
            let left_trap = if i == 0 { false } else { row[i - 1] == '^' };
            let right_trap = if i == width - 1 {
                false
            } else {
                row[i + 1] == '^'
            };
            if left_trap ^ right_trap {
                next_row.push('^');
            } else {
                next_row.push('.');
            }
        }
        row = next_row;
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
