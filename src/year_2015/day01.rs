use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 1 of 2015");
    let line = get_lines()[0].to_string();
    let s1 = line.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    });
    let s2 = line
        .chars()
        .scan(0, |acc, c| {
            *acc += match c {
                '(' => 1,
                ')' => -1,
                _ => 0,
            };
            Some(*acc)
        })
        .position(|x| x == -1)
        .unwrap()
        + 1;
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
