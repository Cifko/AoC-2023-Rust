use json::JsonValue;

use crate::helpers::get_lines;

fn get(data: &JsonValue) -> i64 {
    let mut s = 0;
    match data {
        JsonValue::Number(n) => s += n.to_string().parse::<i64>().unwrap(),
        JsonValue::Array(a) => {
            for v in a {
                s += get(v);
            }
        }
        JsonValue::Object(o) => {
            let mut red = false;
            let mut value = 0;
            for (_, v) in o.iter() {
                if v == "red" {
                    red = true;
                    break;
                }
                value += get(v);
            }
            if !red {
                s += value;
            }
        }
        _ => (),
    }
    s
}

pub fn solve() {
    println!("Day 12 of 2015");
    let lines = get_lines();
    let line = &lines[0];
    let mut s1 = 0;
    let mut s2 = 0;
    let re_nums = regex::Regex::new(r"-?\d+").unwrap();
    for num in re_nums.captures_iter(line) {
        let num = num[0].parse::<i32>().unwrap();
        s1 += num;
    }
    let data = json::parse(line).unwrap();
    s2 = get(&data);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
