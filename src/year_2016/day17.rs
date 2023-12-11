use crate::helpers::get_lines;
use std::{cmp::Reverse, collections::BinaryHeap};

fn md5(passcode: &str, path: &String) -> String {
    format!("{:x}", md5::compute(format!("{}{}", passcode, path)))
}

fn get_opendoors(passcode: &str, path: &String) -> Vec<char> {
    let hash = md5(passcode, path);
    let mut doors = Vec::new();
    if hash[0..1] > *"a" {
        doors.push('U');
    }
    if hash[1..2] > *"a" {
        doors.push('D');
    }
    if hash[2..3] > *"a" {
        doors.push('L');
    }
    if hash[3..4] > *"a" {
        doors.push('R');
    }
    doors
}

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
struct State {
    moves: usize,
    x: i32,
    y: i32,
    path: String,
}

impl State {
    fn new() -> State {
        State {
            moves: 0,
            x: 0,
            y: 0,
            path: "".to_string(),
        }
    }

    fn next_states(&self, passcode: &str) -> Vec<State> {
        let mut next_states = Vec::new();
        let doors = get_opendoors(passcode, &self.path);
        for door in doors {
            match door {
                'U' => {
                    if self.y > 0 {
                        next_states.push(State {
                            moves: self.moves + 1,
                            x: self.x,
                            y: self.y - 1,
                            path: format!("{}{}", self.path, door),
                        })
                    }
                }
                'L' => {
                    if self.x > 0 {
                        next_states.push(State {
                            moves: self.moves + 1,
                            x: self.x - 1,
                            y: self.y,
                            path: format!("{}{}", self.path, door),
                        })
                    }
                }
                'D' => {
                    if self.y < 3 {
                        next_states.push(State {
                            moves: self.moves + 1,
                            x: self.x,
                            y: self.y + 1,
                            path: format!("{}{}", self.path, door),
                        })
                    }
                }
                'R' => {
                    if self.x < 3 {
                        next_states.push(State {
                            moves: self.moves + 1,
                            x: self.x + 1,
                            y: self.y,
                            path: format!("{}{}", self.path, door),
                        })
                    }
                }
                _ => panic!("Unknown door"),
            }
        }
        next_states
    }
}

pub fn solve() {
    println!("Day 17 of 2016");
    let lines = get_lines();
    let passcode = lines[0].clone();
    let s1;
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(State::new()));
    loop {
        let Reverse(state) = queue.pop().unwrap();
        if state.x == 3 && state.y == 3 {
            s1 = state.path;
            break;
        }
        state
            .next_states(passcode.as_str())
            .into_iter()
            .for_each(|s| queue.push(Reverse(s)));
    }
    println!("Part 1: {}", s1);
    let mut s2 = s1.len();
    while !queue.is_empty() {
        let Reverse(state) = queue.pop().unwrap();
        if state.x == 3 && state.y == 3 {
            s2 = s2.max(state.path.len());
            continue;
        }
        state
            .next_states(passcode.as_str())
            .into_iter()
            .for_each(|s| queue.push(Reverse(s)));
    }
    println!("Part 2: {}", s2);
}
