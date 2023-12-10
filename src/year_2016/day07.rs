use crate::helpers::get_lines;

fn has_abba(s: &str) -> bool {
    if s.len() < 4 {
        return false;
    }
    let mut chars = s.chars();
    let mut a = chars.next().unwrap();
    let mut b = chars.next().unwrap();
    let mut c = chars.next().unwrap();
    while let Some(d) = chars.next() {
        if a == d && b == c && a != b {
            return true;
        }
        a = b;
        b = c;
        c = d;
    }
    false
}

// look for aba, return bab
fn get_bab(s: &str) -> Vec<String> {
    let mut res = Vec::new();
    if s.len() < 3 {
        return res;
    }
    let mut chars = s.chars();
    let mut a = chars.next().unwrap();
    let mut b = chars.next().unwrap();
    while let Some(c) = chars.next() {
        if a == c && a != b {
            res.push(format!("{}{}{}", b, a, b));
        }
        a = b;
        b = c;
    }
    res
}

pub fn solve() {
    println!("Day 7 of 2016");
    let lines = get_lines();
    let mut s1 = 0;
    let mut s2 = 0;
    for line in &lines {
        let re = regex::Regex::new(r"(\w*)").unwrap();
        let segments = re.find_iter(line).map(|m| m.as_str()).collect::<Vec<_>>();
        let outter = segments.iter().step_by(2).collect::<Vec<_>>();
        let inner = segments.iter().skip(1).step_by(2).collect::<Vec<_>>();
        if inner.iter().all(|s| !has_abba(s)) && outter.iter().any(|s| has_abba(s)) {
            s1 += 1;
        }
        inner.iter().for_each(|s| {
            let babs = get_bab(s);
            if babs
                .iter()
                .any(|bab| outter.iter().any(|s| s.contains(bab)))
            {
                s2 += 1;
            }
        });
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
