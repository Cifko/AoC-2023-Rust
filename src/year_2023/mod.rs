pub mod day01;
pub mod day02;

pub fn solve(day: u8) {
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        _ => println!("Unknown day {}", day),
    }
}
