use num::integer::Roots;

use crate::helpers::{divisors, gen_primes, get_lines};

pub fn solve() {
    println!("Day 20 of 2015");
    let lines = get_lines();
    let line = &lines[0];
    let house = line.parse::<usize>().unwrap();
    let primes = gen_primes((house / 10).sqrt());
    let mut s1 = 1;
    while divisors(s1, &primes).iter().sum::<usize>() < house / 10 {
        s1 += 1;
    }
    let mut s2 = 1;
    while divisors(s2, &primes)
        .iter()
        .filter(|&x| s2 / x <= 50)
        .sum::<usize>()
        < house / 11
    {
        s2 += 1;
    }
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
