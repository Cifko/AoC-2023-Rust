use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 4 of 2015");
    let line = get_lines()[0].clone();
    let mut s1 = 0;
    while !format!("{:x}", md5::compute(format!("{}{}", line, s1))).starts_with("00000") {
        s1 += 1;
    }
    let mut s2 = s1;
    while !format!("{:x}", md5::compute(format!("{}{}", line, s2))).starts_with("000000") {
        s2 += 1;
    }
    // let hash = Digest::().update(line).finalize();
    // println!("Part 1: {}", line);

    println!("Part 1: {:?}", s1);
    println!("Part 2: {}", s2);
}
