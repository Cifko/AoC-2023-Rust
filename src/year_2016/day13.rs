use std::collections::HashMap;

use crate::helpers::get_lines;

fn is_empty_space(x: u64, y: u64, favorite_number: u64) -> bool {
    let n = (x + y) * (x + y + 1) + 2 * x + favorite_number;
    n.count_ones() % 2 == 0
}

pub fn solve() {
    println!("Day 13 of 2016");
    let lines = get_lines();
    let favorite_number = lines[0].parse::<u64>().unwrap();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut moves = HashMap::new();
    let mut next = vec![(1, 1)];
    let mut cnt = 0;
    while !moves.contains_key(&(31, 39)) || cnt <= 50 {
        let mut next_moves = Vec::new();
        for (x, y) in next {
            moves.insert((x, y), cnt);
            if x > 0
                && is_empty_space(x - 1, y, favorite_number)
                && !moves.contains_key(&(x - 1, y))
            {
                next_moves.push((x - 1, y));
            }
            if is_empty_space(x + 1, y, favorite_number) && !moves.contains_key(&(x + 1, y)) {
                next_moves.push((x + 1, y));
            }
            if y > 0
                && is_empty_space(x, y - 1, favorite_number)
                && !moves.contains_key(&(x, y - 1))
            {
                next_moves.push((x, y - 1));
            }
            if is_empty_space(x, y + 1, favorite_number) && !moves.contains_key(&(x, y + 1)) {
                next_moves.push((x, y + 1));
            }
        }
        next = next_moves;
        if cnt == 50 {
            s2 = moves.len();
        }
        cnt += 1;
    }
    println!("Part 1: {}", moves[&(31, 39)]);
    println!("Part 2: {}", s2);
}
