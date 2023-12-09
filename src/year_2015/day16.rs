use crate::helpers::get_lines;

fn check1(t: &str, c: i32) -> bool {
    match t {
        "children" => c == 3,
        "cats" => c == 7,
        "samoyeds" => c == 2,
        "pomeranians" => c == 3,
        "akitas" => c == 0,
        "vizslas" => c == 0,
        "goldfish" => c == 5,
        "trees" => c == 3,
        "cars" => c == 2,
        "perfumes" => c == 1,
        _ => false,
    }
}

fn check2(t: &str, c: i32) -> bool {
    match t {
        "children" => c == 3,
        "cats" => c > 7,
        "samoyeds" => c == 2,
        "pomeranians" => c < 3,
        "akitas" => c == 0,
        "vizslas" => c == 0,
        "goldfish" => c < 5,
        "trees" => c > 3,
        "cars" => c == 2,
        "perfumes" => c == 1,
        _ => false,
    }
}

pub fn solve() {
    println!("Day 16 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let re = regex::Regex::new(r"Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)").unwrap();
        let (_, [sue, t0, c0, t1, c1, t2, c2]) = re.captures(line).unwrap().extract();
        let c0 = c0.parse::<i32>().unwrap();
        let c1 = c1.parse::<i32>().unwrap();
        let c2 = c2.parse::<i32>().unwrap();
        if check1(&t0, c0) && check1(&t1, c1) && check1(&t2, c2) {
            s1 = sue.parse::<i32>().unwrap();
        }
        if check2(&t0, c0) && check2(&t1, c1) && check2(&t2, c2) {
            s2 = sue.parse::<i32>().unwrap();
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
