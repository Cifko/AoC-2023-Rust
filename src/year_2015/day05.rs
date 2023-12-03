use crate::common::get_lines;

pub fn solve() {
    println!("Day 5 of 2015");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in lines {
        let mut vowels = 0;
        let mut double = false;
        let mut bad = false;
        let mut pair = false;
        let mut repeat = false;
        let mut last = ' ';
        let mut last2 = ' ';
        for c in line.chars() {
            if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' {
                vowels += 1;
            }
            if c == last {
                double = true;
            }
            if (last == 'a' && c == 'b')
                || (last == 'c' && c == 'd')
                || (last == 'p' && c == 'q')
                || (last == 'x' && c == 'y')
            {
                bad = true;
            }
            if last2 == c {
                repeat = true;
            }
            if last != ' ' {
                let mut s = String::new();
                s.push(last);
                s.push(c);
                if line.matches(&s).count() > 1 {
                    pair = true;
                }
            }
            last2 = last;
            last = c;
        }
        if vowels >= 3 && double && !bad {
            s1 += 1;
        }
        if pair && repeat {
            s2 += 1;
        }
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
