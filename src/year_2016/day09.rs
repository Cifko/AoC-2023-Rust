use crate::helpers::get_lines;

fn size2(line: &str, pos: usize) -> (usize, usize, usize) {
    if line.chars().nth(pos).unwrap() != '(' {
        return (1, 1, pos + 1);
    }
    let mut j = pos + 1;
    while line.chars().nth(j).unwrap() != ')' {
        j += 1;
    }
    let mut nums = line[pos + 1..j].split('x');
    let len = nums.next().unwrap().parse::<usize>().unwrap();
    let rep = nums.next().unwrap().parse::<usize>().unwrap();
    let mut k = j + 1;
    let mut s = 0;
    while k < j + 1 + len {
        let (_, len2, j2) = size2(line, k);
        k = j2;
        s += len2;
    }
    (len * rep, rep * s, j + 1 + len)
}

pub fn solve() {
    println!("Day 9 of 2016");
    let lines = get_lines();
    let line = lines[0].trim();
    let mut s1 = 0;
    let mut s2 = 0;
    let mut i = 0;
    while i < line.len() {
        let (len1, len2, j) = size2(line, i);
        s1 += len1;
        s2 += len2;
        i = j;
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
