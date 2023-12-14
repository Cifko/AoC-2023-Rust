use std::fmt::Display;

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Grid<T> {
    pub grid: Vec<Vec<T>>,
}

impl<T: Clone + Default> Grid<T> {
    pub fn new() -> Self {
        Grid { grid: Vec::new() }
    }

    pub fn width(&self) -> usize {
        self.grid[0].len()
    }

    pub fn height(&self) -> usize {
        self.grid.len()
    }

    pub fn add_rows(&mut self, rows: Vec<Vec<T>>) {
        self.grid.extend(rows);
    }

    pub fn add_row(&mut self, row: Vec<T>) {
        self.grid.push(row);
    }

    pub fn add_border(&mut self, value: T) {
        let width = self.grid[0].len();
        let height = self.grid.len();
        self.grid
            .insert(0, (0..width).map(|x| value.clone()).collect::<Vec<_>>());
        self.grid
            .push((0..width).map(|x| value.clone()).collect::<Vec<_>>());
        for row in &mut self.grid {
            row.insert(0, value.clone());
            row.push(value.clone());
        }
    }

    pub fn upscale3x3<F>(&mut self, upscale: F) -> bool
    where
        F: Fn(&T) -> [[T; 3]; 3],
    {
        let mut new_grid: Vec<Vec<T>> =
            vec![vec![T::default(); self.grid[0].len() * 3]; self.grid.len() * 3];
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let upscale = upscale(cell);
                for (iy, row) in upscale.into_iter().enumerate() {
                    for (ix, cell) in row.into_iter().enumerate() {
                        new_grid[iy + y * 3][ix + x * 3] = cell;
                    }
                }
            }
        }
        self.grid = new_grid;
        true
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
        F: Fn(&T, Vec<&T>) -> T,
    {
        let mut new_grid_value = self.grid.clone();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                new_grid_value[y][x] = f(
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
        F: Fn(&T, Vec<&T>) -> T,
    {
        let mut new_grid_value = self.grid.clone();
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                new_grid_value[y][x] = f(
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
            .map(|row| row.iter().filter(|&cell| *cell == value).count())
            .sum()
    }

    pub fn flood_fill(&mut self, x: usize, y: usize, value: T) -> usize
    where
        T: PartialOrd,
    {
        let mut stack = Vec::new();
        let init_value = self.grid[y][x].clone();
        let mut filled = 0;
        stack.push((x, y));
        while let Some((x, y)) = stack.pop() {
            if self.grid[y][x] == value || self.grid[y][x] != init_value {
                continue;
            }
            filled += 1;
            self.grid[y][x] = value.clone();
            for (nx, ny) in self.get4(x, y) {
                stack.push((nx, ny));
            }
        }
        filled
    }

    pub fn find_one(&self, value: T) -> Option<(usize, usize)>
    where
        T: PartialEq,
    {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell == value {
                    return Some((x, y));
                }
            }
        }
        None
    }
}

impl Display for Grid<char> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.grid {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
