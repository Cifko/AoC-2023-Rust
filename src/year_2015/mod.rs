pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub fn solve(day: u8) {
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        _ => println!("Unknown day {}", day),
    }
}
