use std::collections::HashMap;

use crate::helpers::get_lines;

fn md5(salt: &str, id: usize) -> String {
    format!("{:x}", md5::compute(format!("{}{}", salt, id)))
}

fn md5_2(salt: &str, id: usize) -> String {
    let mut hash = format!("{:x}", md5::compute(format!("{}{}", salt, id)));
    for _ in 0..2016 {
        hash = format!("{:x}", md5::compute(hash));
    }
    hash
}

fn get_triplet(s: &str) -> Option<char> {
    let mut chars = s.chars();
    let mut c = chars.next()?;
    let mut count = 1;
    while let Some(n) = chars.next() {
        if n == c {
            count += 1;
            if count == 3 {
                return Some(c);
            }
        } else {
            c = n;
            count = 1;
        }
    }
    None
}

fn has_quintuple(s: &str, l: char) -> bool {
    let mut chars = s.chars();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if c == l {
            count += 1;
            if count == 5 {
                return true;
            }
        } else {
            count = 0;
        }
    }
    false
}

pub fn solve() {
    println!("Day 14 of 2016");
    let lines = get_lines();
    let salt = lines[0].to_string();
    let mut keys = Vec::new();
    let mut hashes = HashMap::new();
    let mut id = 0;
    while keys.len() < 64 {
        let hash = hashes.entry(id).or_insert_with(|| md5(&salt, id));
        if let Some(c) = get_triplet(hash.as_str()) {
            for i in id + 1..id + 1001 {
                let hash = hashes.entry(i).or_insert_with(|| md5(&salt, i));
                if has_quintuple(&hash, c) {
                    keys.push(id);
                    break;
                }
            }
        }
        id += 1;
    }
    println!("Part 1: {}", id - 1);
    let mut id = 0;
    let mut keys = Vec::new();
    let mut hashes = HashMap::new();
    while keys.len() < 64 {
        let hash = hashes.entry(id).or_insert_with(|| md5_2(&salt, id));
        if let Some(c) = get_triplet(hash.as_str()) {
            for i in id + 1..id + 1001 {
                let hash = hashes.entry(i).or_insert_with(|| md5_2(&salt, i));
                if has_quintuple(&hash, c) {
                    keys.push(id);
                    break;
                }
            }
        }
        id += 1;
    }
    println!("Part 2: {}", id - 1);
}
