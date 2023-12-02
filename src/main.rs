use std::env;

mod common;
mod day01;
mod day02;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let day = args[1].parse::<u8>().unwrap();
        match day {
            1 => day01::solve(),
            2 => day02::solve(),
            _ => println!("Unknown day {}", day),
        }
    } else {
        println!("Please specify a day e.g. day 1 is `aoc-2023 1`");
    }
}
