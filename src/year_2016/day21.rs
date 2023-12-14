use crate::helpers::{get_lines, parse};

pub fn solve() {
    println!("Day 21 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    // let mut password = "abcdefgh";
    // for line in &lines {
    //     let (parts, nums) = parse(line);
    //     let mut snums = nums;
    //     snums.sort();
    //     match parts[0] {
    //         "swap" => match parts[1] {
    //             "position" => {
    //                 password = password[0..nums[0]] + password[nums[0]+1)]
    //             },
    //             _ => panic!("Unknown instruction: {}", parts[1]),
    //         },
    //         _ => panic!("Unknown instruction: {}", parts[0]),
    //     }
    // }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
