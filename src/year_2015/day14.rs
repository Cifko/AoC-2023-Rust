use std::collections::HashMap;

use crate::helpers::get_lines;

fn distance(speed: i64, fly_time: i64, rest_time: i64, time: i64) -> i64 {
    let cycle_time = fly_time + rest_time;
    let cycles = time / cycle_time;
    let remainder = time % cycle_time;
    let fly_time = fly_time * cycles + remainder.min(fly_time);
    speed * fly_time
}

pub fn solve() {
    println!("Day 14 of 2015");
    let lines = get_lines();
    let mut reindeers = HashMap::new();
    for line in &lines {
        let re = regex::Regex::new(
            r"(\w+) can fly (\d+) km/s for (\d+) seconds, but then must rest for (\d+) seconds.",
        )
        .unwrap();
        let (_, [who, speed, fly_time, rest_time]) = re.captures(line.as_str()).unwrap().extract();
        let speed = speed.parse::<i64>().unwrap();
        let fly_time = fly_time.parse::<i64>().unwrap();
        let rest_time = rest_time.parse::<i64>().unwrap();
        reindeers.insert(who, (speed, fly_time, rest_time));
    }
    let mut s1 = 0;
    for (_, (speed, fly_time, rest_time)) in &reindeers {
        s1 = s1.max(distance(*speed, *fly_time, *rest_time, 2503));
    }
    let mut s2 = 0;
    let mut points = HashMap::new();
    for i in 1..=2503 {
        let mut m = 0;
        for (_, (speed, fly_time, rest_time)) in &reindeers {
            m = m.max(distance(*speed, *fly_time, *rest_time, i));
        }
        for (name, (speed, fly_time, rest_time)) in &reindeers {
            if m == distance(*speed, *fly_time, *rest_time, i) {
                *points.entry(name).or_insert(0) += 1;
            }
        }
    }
    s2 = points.values().max().unwrap().to_owned();
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
