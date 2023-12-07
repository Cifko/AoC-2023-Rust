use crate::helpers::get_lines;

fn is_password_valid(password: &String) -> bool {
    let mut has_straight = false;
    let mut last_char = ' ';
    let mut checked = false;
    let mut prev_last_char = ' ';
    let mut cnt = 0;
    let mut pairs = 0;
    for c in password.chars() {
        if c == 'i' || c == 'o' || c == 'l' {
            // println!("{} is invalid", password);
            return false;
        }
        if c == last_char && !checked {
            pairs += 1;
            checked = true;
        } else {
            checked = false;
        }
        if prev_last_char as u8 + 1 == last_char as u8 && last_char as u8 + 1 == c as u8 {
            has_straight = true;
        }
        prev_last_char = last_char;
        last_char = c;
    }
    has_straight && pairs >= 2
}

fn increment_password(password: String) -> String {
    let mut pass = password.chars().collect::<Vec<char>>();
    for i in (0..pass.len()).rev() {
        if pass[i] == 'z' {
            pass[i] = 'a';
        } else {
            pass[i] = (pass[i] as u8 + 1) as char;
            return String::from_iter(pass);
        }
    }
    panic!();
}

pub fn solve() {
    println!("Day 11 of 2015");
    let lines = get_lines();
    let mut line = lines[0].clone();
    loop {
        line = increment_password(line);
        if is_password_valid(&line) {
            break;
        }
    }
    println!("Part 1: {}", line);
    loop {
        line = increment_password(line);
        if is_password_valid(&line) {
            break;
        }
    }
    println!("Part 2: {}", line);
}
