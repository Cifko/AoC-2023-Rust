use crate::helpers::get_lines;

pub fn solve() {
    println!("Day 3 of 2015");
    let line = get_lines()[0].to_string();
    let s1 = line
        .chars()
        .scan((0, 0), |(x, y), c| {
            *x += match c {
                '<' => -1,
                '>' => 1,
                _ => 0,
            };
            *y += match c {
                '^' => 1,
                'v' => -1,
                _ => 0,
            };
            Some((*x, *y))
        })
        .chain(vec![(0, 0)].into_iter()) // Add starting location
        .collect::<Vec<(i32, i32)>>()
        .iter()
        .collect::<std::collections::HashSet<&(i32, i32)>>()
        .len();
    let s2 = line
        .chars()
        .enumerate()
        .scan(((0, 0), (0, 0)), |((x1, y1), (x2, y2)), (i, c)| {
            if i % 2 == 0 {
                *x1 += match c {
                    '<' => -1,
                    '>' => 1,
                    _ => 0,
                };
                *y1 += match c {
                    '^' => 1,
                    'v' => -1,
                    _ => 0,
                };
                Some((*x1, *y1))
            } else {
                *x2 += match c {
                    '<' => -1,
                    '>' => 1,
                    _ => 0,
                };
                *y2 += match c {
                    '^' => 1,
                    'v' => -1,
                    _ => 0,
                };
                Some((*x2, *y2))
            }
        })
        .chain(vec![(0, 0)].into_iter()) // Add starting location
        .collect::<Vec<(i32, i32)>>()
        .iter()
        .collect::<std::collections::HashSet<&(i32, i32)>>()
        .len();
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
