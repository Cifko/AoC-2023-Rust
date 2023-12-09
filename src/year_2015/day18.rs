use crate::helpers::{get_lines, Grid};

pub fn solve() {
    println!("Day 18 of 2015");
    let lines = get_lines();
    let mut grid = Grid::new();
    for line in &lines {
        grid.add_row(line.chars().collect());
    }
    let mut grid2 = grid.clone();
    grid2.grid[0][0].value = '#';
    grid2.grid[grid.grid.len() - 1][0].value = '#';
    grid2.grid[grid.grid.len() - 1][grid.grid[0].len() - 1].value = '#';
    grid2.grid[0][grid.grid[0].len() - 1].value = '#';
    for _ in 0..100 {
        grid.apply_8(|cell, neighbours| {
            let on = neighbours
                .into_iter()
                .filter(|neighbour| neighbour.value == '#')
                .count();
            if on == 3 || (cell.value == '#' && on == 2) {
                '#'
            } else {
                '.'
            }
        });
        grid2.apply_8(|cell, neighbours| {
            if neighbours.len() == 3 {
                return '#';
            }
            let on = neighbours
                .into_iter()
                .filter(|neighbour| neighbour.value == '#')
                .count();
            if on == 3 || (cell.value == '#' && on == 2) {
                '#'
            } else {
                '.'
            }
        });
    }
    let s1 = grid.count('#');
    let s2 = grid2.count('#');
    println!("Part 1: {}", s1);
    println!("Part 2: {}", s2);
}
