use regex::Regex;
use std::io::{self};

pub fn get_lines() -> Vec<String> {
    let mut input = String::new();
    let mut lines = Vec::new();
    while let Ok(line) = io::stdin().read_line(&mut input) {
        if line == 0 {
            break;
        }
        lines.push(input.trim().to_string());
        input.clear();
    }
    lines
}

pub fn find_overlapping(input: &str, pattern: &str) -> Vec<String> {
    let re = Regex::new(pattern).unwrap();
    let mut start = 0;
    let mut results = Vec::new();
    while let Some(found) = re.find_at(input, start) {
        results.push(found.as_str().to_string());
        start = found.start() + 1;
    }

    results
}
