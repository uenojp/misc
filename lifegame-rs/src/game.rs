#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Dead,
    Alive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rotate {
    R0,
    R90,
    R180,
    R270,
}

#[derive(Debug)]
pub struct Game {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(height: usize, width: usize) -> Self {
        Self {
            cells: vec![vec![Cell::Dead; width]; height],
            height,
            width,
        }
    }

    pub fn next(&mut self) {
        let mut next = vec![vec![Cell::Dead; self.width]; self.height];

        for x in 0..self.height {
            for y in 0..self.width {
                let neighbors = self.count_neighbors(x, y);

                if let Some(cell) = self.get(x, y) {
                    let next_cell = match (*cell, neighbors) {
                        (Cell::Dead, 3) => Cell::Alive,
                        (Cell::Alive, 2 | 3) => Cell::Alive,
                        _ => Cell::Dead,
                    };
                    next[x][y] = next_cell;
                }
            }
        }

        self.cells = next;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let nx = x as isize + i;
                let ny = y as isize + j;
                if nx < 0 || self.height as isize <= nx || ny < 0 || self.width as isize <= ny {
                    continue;
                }
                if let Some(cell) = self.get(nx as usize, ny as usize) {
                    if *cell == Cell::Alive {
                        count += 1;
                    }
                }
            }
        }
        count
    }

    fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(x).and_then(|row| row.get(y))
    }

    fn set(&mut self, x: usize, y: usize, cell: Cell) -> bool {
        if let Some(c) = self.cells.get_mut(x).and_then(|row| row.get_mut(y)) {
            *c = cell;
            return true;
        }
        false
    }

    pub fn place_rotated_pattern<Pattern: AsRef<[Row]>, Row: AsRef<[Cell]>>(
        &mut self,
        pattern: Pattern,
        x: usize,
        y: usize,
        rotate: Rotate,
    ) {
        let rotated = match rotate {
            Rotate::R0 => {
                self.place_pattern(&pattern, x, y);
                return;
            }
            Rotate::R90 => self.rotate_pattern(&pattern),
            Rotate::R180 => self.rotate_pattern(&self.rotate_pattern(&pattern)),
            Rotate::R270 => {
                self.rotate_pattern(&self.rotate_pattern(&self.rotate_pattern(&pattern)))
            }
        };
        self.place_pattern(&rotated, x, y);
    }

    fn rotate_pattern<Pattern: AsRef<[Row]>, Row: AsRef<[Cell]>>(
        &self,
        pattern: Pattern,
    ) -> Vec<Vec<Cell>> {
        let w = pattern
            .as_ref()
            .get(0)
            .map(|row| row.as_ref().len())
            .unwrap_or(0);
        let h = pattern.as_ref().len();
        let mut rotated = vec![vec![Cell::Dead; h]; w];
        for (i, row) in pattern.as_ref().iter().enumerate() {
            for (j, cell) in row.as_ref().iter().enumerate() {
                rotated[j][h - 1 - i] = *cell;
            }
        }
        rotated
    }

    pub fn place_pattern<Pattern: AsRef<[Row]>, Row: AsRef<[Cell]>>(
        &mut self,
        pattern: Pattern,
        x: usize,
        y: usize,
    ) {
        for (i, row) in pattern.as_ref().iter().enumerate() {
            for (j, cell) in row.as_ref().iter().enumerate() {
                // set(x + i, y + j, cell | cells[x + i][y + j]);
                let cell = if let (Some(Cell::Dead), Cell::Dead) = (self.get(x + i, y + j), cell) {
                    Cell::Dead
                } else {
                    Cell::Alive
                };
                self.set(x + i, y + j, cell);
            }
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.height, self.width)
    }

    pub fn iter(&self) -> impl Iterator<Item = &Vec<Cell>> {
        self.cells.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const D: Cell = Cell::Dead;
    const A: Cell = Cell::Alive;

    #[test]
    fn all() {
        let mut g = Game::new(3, 3);

        // .#.
        // .#.
        // .#.
        g.cells = vec![vec![D, A, D], vec![D, A, D], vec![D, A, D]];
        assert_eq!(2, g.count_neighbors(0, 0));
        assert_eq!(1, g.count_neighbors(0, 1));
        assert_eq!(3, g.count_neighbors(1, 0));
        assert_eq!(2, g.count_neighbors(1, 1));
        assert_eq!(3, g.count_neighbors(1, 2));
        assert_eq!(2, g.count_neighbors(2, 0));
        assert_eq!(2, g.count_neighbors(2, 2));

        g.next();
        assert_eq!(vec![vec![D, D, D], vec![A, A, A], vec![D, D, D]], g.cells);

        g.next();
        assert_eq!(vec![vec![D, A, D], vec![D, A, D], vec![D, A, D]], g.cells);

        // ###
        // ###
        // ###
        g.cells = vec![vec![A, A, A], vec![A, A, A], vec![A, A, A]];
        g.next();
        assert_eq!(vec![vec![A, D, A], vec![D, D, D], vec![A, D, A]], g.cells);
    }
}
