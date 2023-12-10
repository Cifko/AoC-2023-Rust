use crate::helpers::get_lines;

fn find_next(i: usize, line: &str) -> (usize, String) {
    let mut i = i;
    while !format!("{:x}", md5::compute(format!("{}{}", line, i))).starts_with("00000") {
        i += 1;
    }
    let hash = format!("{:x}", md5::compute(format!("{}{}", line, i)));
    (i, hash)
}

pub fn solve() {
    println!("Day 5 of 2015");
    let lines = get_lines();
    let line = &lines[0];
    let mut s1 = String::new();
    let mut s2 = vec![None; 8];
    // let line = "abc";
    let mut i = 0;
    for _ in 0..8 {
        let (j, hash) = find_next(i, line);
        i = j + 1;
        let fifth = hash.chars().nth(5).unwrap();
        s1 += &fifth.to_string();
        if fifth.is_digit(8) {
            let pos = fifth.to_digit(8).unwrap() as usize;
            if s2[pos].is_none() {
                s2[pos] = Some(hash.chars().nth(6).unwrap());
            }
        }
    }
    println!("Part 1: {}", s1);
    while s2.iter().any(|s| s.is_none()) {
        let (j, hash) = find_next(i, line);
        i = j + 1;
        let fifth = hash.chars().nth(5).unwrap();
        if fifth.is_digit(8) {
            let pos = fifth.to_digit(8).unwrap() as usize;
            if s2[pos].is_none() {
                s2[pos] = Some(hash.chars().nth(6).unwrap());
            }
        }
    }
    println!(
        "Part 2: {}",
        s2.iter().map(|s| s.unwrap()).collect::<String>()
    );
}
