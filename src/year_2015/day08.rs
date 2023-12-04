use crate::helpers::get_lines;


pub fn solve() {
    println!("Day 8 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let mut skip = 0;
        let mut last_char = ' ';
        s1 += 2;
        s2 += 2;
        for c in line.chars() {
            match c {
                '\\' => s2 += 1,
                '"' => s2 += 1,
                _ => (),
            }
            if last_char == '\\' {
                match c {
                    'x' => s1 += 3,
                    '"' => s1 += 1,
                    '\\' => s1 += 1,
                    _ => (),
                }
                last_char = ' ';
            } else {
                last_char = c;
            }
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
