use std::{cmp::Reverse, collections::BinaryHeap};

use crate::helpers::get_lines;

#[derive(Eq, PartialEq, PartialOrd, Ord)]
struct State {
    number_of_packages: usize,
    quantum_entalment: i64,
    target_weight: i64,
    next_package_to_try: usize,
}

fn find_quantum_entanglement(packages: &[i64], target_weight: i64) -> i64 {
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State {
        number_of_packages: 0,
        quantum_entalment: 1,
        target_weight,
        next_package_to_try: 0,
    }));
    loop {
        let Reverse(state) = queue.pop().unwrap();
        if state.target_weight == 0 {
            return state.quantum_entalment;
        }
        for i in state.next_package_to_try..packages.len() {
            let new_state = State {
                number_of_packages: state.number_of_packages + 1,
                quantum_entalment: state.quantum_entalment * packages[i],
                target_weight: state.target_weight - packages[i],
                next_package_to_try: i + 1,
            };
            if new_state.target_weight < 0 {
                continue;
            }
            queue.push(Reverse(new_state));
        }
    }
}

pub fn solve() {
    println!("Day 24 of 2015");
    let lines = get_lines();
    let mut packages = Vec::new();
    for line in &lines {
        packages.push(line.parse::<i64>().unwrap());
    }
    let s1 = find_quantum_entanglement(&packages, packages.iter().sum::<i64>() / 3);
    let s2 = find_quantum_entanglement(&packages, packages.iter().sum::<i64>() / 4);
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
