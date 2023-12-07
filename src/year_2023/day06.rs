use crate::helpers::get_lines;

fn race(time: u64, distance: u64) -> u64 {
    let mut s = 0;
    for t in 1..time {
        if (time - t) * t > distance {
            s += 1;
        }
    }
    s
}

pub fn solve() {
    println!("Day 06 of 2023");
    let lines = get_lines();
    let mut s1 = 1;
    let mut s2 = 0;
    let re_nums = regex::Regex::new(r"(\d+)").unwrap();
    let times = re_nums
        .find_iter(&lines[0])
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let time2 = re_nums
        .find_iter(&lines[0])
        .fold("".to_string(), |acc, x| acc + x.as_str());
    let time2 = time2.parse::<u64>().unwrap();
    let distances = re_nums
        .find_iter(&lines[1])
        .map(|x| x.as_str().parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let distance2 = re_nums
        .find_iter(&lines[1])
        .fold("".to_string(), |acc, x| acc + x.as_str());
    let distance2 = distance2.parse::<u64>().unwrap();
    for (time, distance) in times.iter().zip(distances.iter()) {
        s1 *= race(*time, *distance);
    }
    s2 = race(time2, distance2);
    println!("Part 1 {}", s1);
    println!("Part 2 {}", s2);
}
