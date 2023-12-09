use crate::helpers::get_lines;

fn solve1(containers: &[usize], target: usize) -> usize {
    if target == 0 {
        return 1;
    }
    if containers.len() == 0 {
        return 0;
    }
    let mut count = 0;
    if target >= containers[0] {
        count += solve1(&containers[1..], target - containers[0]);
    }
    count += solve1(&containers[1..], target);
    return count;
}

fn solve2(containers: &[usize], target: usize, used: usize) -> (usize, usize) {
    if target == 0 {
        return (1, used);
    }
    if containers.len() == 0 {
        return (0, usize::MAX);
    }
    let mut count = 0;
    let mut min_used = usize::MAX;
    if target >= containers[0] {
        let (cnt, usd) = solve2(&containers[1..], target - containers[0], used + 1);
        if usd < min_used {
            min_used = usd;
            count = 0;
        }
        if usd == min_used {
            count += cnt;
        }
    }
    let (cnt, usd) = solve2(&containers[1..], target, used);
    if usd < min_used {
        min_used = usd;
        count = 0;
    }
    if usd == min_used {
        count += cnt;
    }
    return (count, min_used);
}

pub fn solve() {
    println!("Day 17 of 2015");
    let lines = get_lines();
    let mut containers = Vec::new();
    for line in &lines {
        containers.push(line.parse::<usize>().unwrap());
    }
    containers.sort();
    containers.reverse();
    let s1 = solve1(&containers, 150);
    let (s2, _) = solve2(&containers, 150, 0);
    println!("{:?}", containers);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
