use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 10 of 2015");
    let lines = get_lines();
    let mut line = lines[0]
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect::<Vec<_>>();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut last_char = 10;
    for _ in 0..40 {
        let mut new_line = Vec::new();
        let mut cnt = 0;
        for c in line {
            if c != last_char {
                if cnt > 0 {
                    new_line.push(cnt);
                    new_line.push(last_char);
                }
                cnt = 0;
            }
            last_char = c;
            cnt += 1;
        }
        new_line.push(cnt);
        new_line.push(last_char);
        line = new_line;
    }
    println!("Part 1: {}", line.len());
    for _ in 0..10 {
        let mut new_line = Vec::new();
        let mut cnt = 0;
        for c in line {
            if c != last_char {
                if cnt > 0 {
                    new_line.push(cnt);
                    new_line.push(last_char);
                }
                cnt = 0;
            }
            last_char = c;
            cnt += 1;
        }
        new_line.push(cnt);
        new_line.push(last_char);
        line = new_line;
    }
    println!("Part 2: {}", line.len());
}
