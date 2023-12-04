use crate::helpers::{find_overlapping, get_lines};

pub fn to_number(s: &String) -> u64 {
    match s.as_str() {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.parse::<u64>().unwrap(),
    }
}

pub fn solve() {
    println!("Day 01");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        // let x = num.find_iter(&line);
        let x = find_overlapping(line, r"\d");
        s1 += to_number(x.first().unwrap()) * 10 + to_number(x.last().unwrap());
        let x = find_overlapping(line, r"\d|one|two|three|four|five|six|seven|eight|nine|ten");
        s2 += to_number(x.first().unwrap()) * 10 + to_number(x.last().unwrap());
    }
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
