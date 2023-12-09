use crate::helpers::{get_lines, parse};

pub fn solve() {
    println!("Day 3 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut triangles = vec![Vec::<i64>::new(); lines.len()];
    for (i, line) in lines.iter().enumerate() {
        let (_, mut nums) = parse(line);
        triangles[i / 3 * 3].push(nums[0]);
        triangles[i / 3 * 3 + 1].push(nums[1]);
        triangles[i / 3 * 3 + 2].push(nums[2]);
        nums.sort();
        if nums[0] + nums[1] > nums[2] {
            s1 += 1;
        }
    }
    let mut s2 = 0;
    for triangle in triangles {
        let mut nums = triangle;
        nums.sort();
        if nums[0] + nums[1] > nums[2] {
            s2 += 1;
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
