use std::fmt::Display;

#[derive(Clone)]
pub struct Grid<T> {
    pub grid: Vec<Vec<Cell<T>>>,
}

#[derive(Clone)]
pub struct Cell<T> {
    pub value: T,
    pub x: usize,
    pub y: usize,
}

impl<T> Cell<T> {
    pub fn new(value: T, x: usize, y: usize) -> Self {
        Cell { value, x, y }
    }
}

// impl<'a, T> Cell<'a, T> {}

impl<T: Clone> Grid<T> {
    pub fn new() -> Self {
        Grid { grid: Vec::new() }
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        // if let Some(mut prev_row) = self.grid.last() {
        //     for cell in prev_row {
        //         // cell.n
        //     }
        // }
        self.grid.push(
            row.into_iter()
                .enumerate()
                .map(|(x, value)| Cell::new(value, x, self.grid.len()))
                .collect::<Vec<_>>(),
        );
    }

    pub fn get4(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut cells = Vec::new();
        if x > 0 {
            cells.push((x - 1, y));
        }
        if x < self.grid[y].len() - 1 {
            cells.push((x + 1, y));
        }
        if y > 0 {
            cells.push((x, y - 1));
        }
        if y < self.grid.len() - 1 {
            cells.push((x, y + 1));
        }
        cells
    }

    pub fn get8(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut cells = self.get4(x, y);
        if x > 0 && y > 0 {
            cells.push((x - 1, y - 1));
        }
        if x > 0 && y < self.grid.len() - 1 {
            cells.push((x - 1, y + 1));
        }
        if x < self.grid[y].len() - 1 && y > 0 {
            cells.push((x + 1, y - 1));
        }
        if x < self.grid[y].len() - 1 && y < self.grid.len() - 1 {
            cells.push((x + 1, y + 1));
        }
        cells
    }

    pub fn apply_4<F>(&mut self, f: F)
    where
        F: Fn(&Cell<T>, Vec<&Cell<T>>) -> T,
    {
        let mut new_grid_value = self.grid.clone();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                new_grid_value.get_mut(y).unwrap().get_mut(x).unwrap().value = f(
                    cell,
                    self.get4(x, y)
                        .into_iter()
                        .map(|(x, y)| &self.grid[y][x])
                        .collect(),
                );
            }
        }
        self.grid = new_grid_value;
    }

    pub fn apply_8<F>(&mut self, f: F)
    where
        F: Fn(&Cell<T>, Vec<&Cell<T>>) -> T,
    {
        let mut new_grid_value = self.grid.clone();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                new_grid_value[y][x].value = f(
                    cell,
                    self.get8(x, y)
                        .into_iter()
                        .map(|(x, y)| &self.grid[y][x])
                        .collect(),
                );
            }
        }
        self.grid = new_grid_value;
    }

    pub fn count(&self, value: T) -> usize
    where
        T: PartialEq,
    {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|cell| cell.value == value).count())
            .sum()
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell.value)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
