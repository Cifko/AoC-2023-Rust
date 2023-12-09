use crate::helpers::get_lines;

fn compute(a: i32, lines: &Vec<String>) -> i32 {
    let mut registers = [a, 0];
    let mut ip = 0;
    loop {
        if ip >= lines.len() {
            return registers[1];
        }
        let line = &lines[ip];
        match line.split_once(' ') {
            Some(("hlf", reg)) => {
                registers[(reg == "b") as usize] /= 2;
                ip += 1;
            }
            Some(("tpl", reg)) => {
                registers[(reg == "b") as usize] *= 3;
                ip += 1;
            }
            Some(("inc", reg)) => {
                registers[(reg == "b") as usize] += 1;
                ip += 1;
            }
            Some(("jmp", offset)) => {
                ip = (ip as i32 + offset.parse::<i32>().unwrap()) as usize;
            }
            Some(("jie", jump)) => {
                let (reg, offset) = jump.split_once(',').unwrap();
                if registers[(reg == "b") as usize] % 2 == 0 {
                    ip = (ip as i32 + offset.trim().parse::<i32>().unwrap()) as usize;
                } else {
                    ip += 1;
                }
            }
            Some(("jio", jump)) => {
                let (reg, offset) = jump.split_once(',').unwrap();
                if registers[(reg == "b") as usize] == 1 {
                    ip = (ip as i32 + offset.trim().parse::<i32>().unwrap()) as usize;
                } else {
                    ip += 1;
                }
            }
            _ => panic!("Invalid instruction"),
        }
    }
}

pub fn solve() {
    println!("Day 23 of 2015");
    let lines = get_lines();
    let s1 = compute(0, &lines);
    let s2 = compute(1, &lines);

    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
