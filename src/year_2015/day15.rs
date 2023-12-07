use crate::helpers::get_lines;

fn solve1(spoons_left: i64, vals: &Vec<Vec<i64>>, selected: &Vec<i64>) -> i64 {
    if selected.len() == vals.len() {
        if spoons_left == 0 {
            let mut m = 1;
            for i in 0..(vals[0].len() - 1) {
                let mut v = 0;
                for j in 0..selected.len() {
                    v += vals[j][i] * selected[j];
                }
                if v < 0 {
                    return 0;
                }
                m *= v;
            }
            return m;
        }
        return 0;
    }
    let mut m = 0;
    let mut new_selected = selected.clone();
    new_selected.push(0);
    for i in 0..=spoons_left {
        new_selected[selected.len()] = i;
        m = m.max(solve1(spoons_left - i, vals, &new_selected));
    }
    m
}

fn solve2(spoons_left: i64, vals: &Vec<Vec<i64>>, selected: &Vec<i64>) -> i64 {
    if selected.len() == vals.len() {
        if spoons_left == 0 {
            let mut calories = 0;
            for i in 0..selected.len() {
                calories += vals[i][vals[i].len() - 1] * selected[i];
            }
            if calories != 500 {
                return 0;
            }
            let mut m = 1;
            for i in 0..(vals[0].len() - 1) {
                let mut v = 0;
                for j in 0..selected.len() {
                    v += vals[j][i] * selected[j];
                }
                if v < 0 {
                    return 0;
                }
                m *= v;
            }
            return m;
        }
        return 0;
    }
    let mut m = 0;
    let mut new_selected = selected.clone();
    new_selected.push(0);
    for i in 0..=spoons_left {
        new_selected[selected.len()] = i;
        m = m.max(solve2(spoons_left - i, vals, &new_selected));
    }
    m
}

pub fn solve() {
    println!("Day 15 of 2015");
    let lines = get_lines();
    let mut vals = Vec::new();
    for line in &lines {
        let re = regex::Regex::new(r"(-?\d+)").unwrap();
        let val = re
            .find_iter(line.as_str())
            .map(|x| x.as_str().parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        vals.push(val);
    }
    let s1 = solve1(100, &vals, &Vec::new());
    let s2 = solve2(100, &vals, &Vec::new());
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
