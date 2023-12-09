use std::collections::{HashMap, HashSet};

use crate::helpers::{get_lines, parse};

fn apply(formulas: &HashMap<usize, Vec<Vec<usize>>>, molecule: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for (i, atom) in molecule.iter().enumerate() {
        if let Some(tos) = formulas.get(atom) {
            for to in tos {
                let new_molecule = [&molecule[..i], &to, &molecule[i + 1..]].concat();
                result.push(new_molecule);
            }
        }
    }
    result
}

fn apply_rev(formulas: &HashMap<usize, Vec<Vec<usize>>>, molecule: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    for (&from, tos) in formulas {
        for to in tos {
            for i in 0..molecule.len() {
                if molecule[i..].starts_with(to) {
                    let new_molecule =
                        [&molecule[..i], &[from], &molecule[i + to.len()..]].concat();
                    result.push(new_molecule);
                }
            }
        }
    }
    result
}

pub fn solve() {
    println!("Day 19 of 2015");
    let lines = get_lines();
    let mut s2 = 0;
    let mut formulas = HashMap::new();
    let re = regex::Regex::new(r"([A-Z][a-z]?)").unwrap();
    let mut mapping = HashMap::new();
    for line in &lines {
        if line.is_empty() {
            break;
        }
        let (parts, _) = parse(line);
        let (from, to) = (parts[0], parts[2]);
        let map_len = mapping.len();
        mapping.entry(from).or_insert(map_len);
        let tos = re.find_iter(to).map(|m| m.as_str()).collect::<Vec<_>>();
        for to in &tos {
            let map_len = mapping.len();
            mapping.entry(to).or_insert(map_len);
        }
        formulas
            .entry(mapping[from])
            .or_insert_with(Vec::new)
            .push(tos.iter().map(|to| mapping[to]).collect::<Vec<_>>());
    }
    let molecule = lines.last().unwrap();
    let molecule = re
        .find_iter(molecule)
        .map(|m| mapping[m.as_str()])
        .collect::<Vec<_>>();
    // let mut seen = std::collections::HashSet::new();
    let s1 = apply(&formulas, &molecule);
    let s1: HashSet<&Vec<usize>> = HashSet::from_iter(s1.iter());
    let s1 = s1.len();
    let x: HashSet<Vec<usize>> = HashSet::from_iter(vec![vec![mapping["e"]]]);
    let mut pos = x.clone();
    loop {
        let new_pos = pos
            .iter()
            .map(|pos| apply(&formulas, pos))
            .collect::<Vec<_>>();
        pos = HashSet::from_iter(
            new_pos
                .into_iter()
                .flatten()
                .filter(|x| x.len() <= molecule.len())
                .collect::<Vec<_>>(),
        );
        // .flatten()
        // .collect::<Vec<_>>();
        println!("s2 {s2} pos {:?}", pos.len());
        s2 += 1;
        if pos
            .iter()
            .any(|pos| pos.len() == molecule.len() && pos == &molecule)
        {
            break;
        }
        // return;
    }

    // let x = vec![molecule.clone()];
    // let mut pos = x.clone();
    // println!("{:?}", molecule);
    // loop {
    //     let new_pos = pos
    //         .iter()
    //         .map(|pos| apply_rev(&formulas, pos))
    //         .collect::<Vec<_>>();
    //     pos = new_pos.into_iter().flatten().collect::<Vec<_>>().clone();
    //     // .flatten()
    //     // .collect::<Vec<_>>();
    //     println!("s2 {s2} pos {:?}", pos.len());
    //     s2 += 1;
    //     if pos
    //         .iter()
    //         .any(|pos| pos.len() == 1 && pos[0] == mapping["e"])
    //     {
    //         break;
    //     }
    //     // return;
    // }

    // let mut queue = BinaryHeap::new();
    // queue.push(Reverse((0, molecule)));
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
