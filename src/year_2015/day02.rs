use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 2 of 2015");
    let lines = get_lines();
    let s1 = lines
        .iter()
        .map(|line| {
            let mut v = line
                .split('x')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            v.sort();
            3 * v[0] * v[1] + 2 * v[0] * v[2] + 2 * v[1] * v[2]
        })
        .sum::<u64>();
    let s2 = lines
        .iter()
        .map(|line| {
            let mut v = line
                .split('x')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            v.sort();
            2 * v[0] + 2 * v[1] + v[0] * v[1] * v[2]
        })
        .sum::<u64>();
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
