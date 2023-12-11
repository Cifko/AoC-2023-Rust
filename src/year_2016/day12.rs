use crate::helpers::{get_lines, parse};

enum Value {
    Register(usize),
    Value(i64),
}

impl From<&str> for Value {
    fn from(s: &str) -> Self {
        match s.parse::<i64>() {
            Ok(v) => Value::Value(v),
            Err(_) => Value::Register(to_register_num(s)),
        }
    }
}

impl Value {
    fn get(&self, registers: &[i64]) -> i64 {
        match self {
            Value::Register(r) => registers[*r],
            Value::Value(v) => *v,
        }
    }
}

enum Instruction {
    Cpy(Value, usize),
    Inc(usize),
    Dec(usize),
    Jnz(Value, Value),
}

fn to_register_num(s: &str) -> usize {
    s.chars().next().unwrap() as usize - 'a' as usize
}

fn run(ignition_key: i64, instructions: &Vec<Instruction>) -> i64 {
    let mut registers = [0; 4];
    registers[2] = ignition_key;
    let mut ip = 0;
    while ip < instructions.len() {
        match &instructions[ip] {
            Instruction::Cpy(x, y) => registers[*y] = x.get(&registers),
            Instruction::Inc(r) => registers[*r] += 1,
            Instruction::Dec(r) => registers[*r] -= 1,
            Instruction::Jnz(x, y) => {
                if x.get(&registers) != 0 {
                    ip = (ip as i64 + y.get(&registers) - 1) as usize;
                }
            }
        }
        ip += 1;
    }
    registers[0]
}

pub fn solve() {
    println!("Day 12 of 2016");
    let lines = get_lines();
    let mut instructions = Vec::new();
    for line in &lines {
        let (parts, _) = parse(line);
        let instruction = match parts[0] {
            "cpy" => Instruction::Cpy(parts[1].into(), to_register_num(parts[2])),
            "inc" => Instruction::Inc(to_register_num(parts[1])),
            "dec" => Instruction::Dec(to_register_num(parts[1])),
            "jnz" => Instruction::Jnz(parts[1].into(), parts[2].into()),
            _ => panic!("Unknown instruction"),
        };
        instructions.push(instruction);
    }
    println!("Part 1: {}", run(0, &instructions));
    println!("Part 2: {}", run(1, &instructions));
}
