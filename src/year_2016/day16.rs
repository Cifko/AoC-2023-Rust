use crate::helpers::get_lines;

fn check_sum(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars();
    while let Some(c1) = chars.next() {
        let c2 = chars.next().unwrap();
        output.push(if c1 == c2 { '1' } else { '0' });
    }
    if output.len() % 2 == 0 {
        check_sum(&output)
    } else {
        output
    }
}

pub fn solve() {
    println!("Day 16 of 2016");
    let lines = get_lines();
    let mut input = lines[0].clone();
    let mut s1 = 0;
    let mut s2 = 0;
    while input.len() < 272 {
        let mut input2 = input.clone();
        input2.push('0');
        for c in input.chars().rev() {
            input2.push(if c == '0' { '1' } else { '0' });
        }
        input = input2;
    }
    let s1 = check_sum(input.as_str()[..272].into());
    while input.len() < 35651584 {
        let mut input2 = input.clone();
        input2.push('0');
        for c in input.chars().rev() {
            input2.push(if c == '0' { '1' } else { '0' });
        }
        input = input2;
    }
    let s2 = check_sum(input.as_str()[..35651584].into());
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
