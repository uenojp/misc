pub struct Grid {
    grid: Vec<Vec<u64>>,
}

impl Grid {
    pub fn new(s: &str) -> Self {
        let grid = s
            .lines()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u64)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Self { grid }
    }

    pub fn is_visible(&self, x: usize, y: usize) -> bool {
        let height = self.grid.len();
        let width = self.grid[0].len();

        if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
            return true;
        }

        let current_tree = self.grid[y][x];

        let row = self.grid[y].clone();
        let column = (0..height).map(|j| self.grid[j][x]).collect::<Vec<_>>();

        column[0..y].iter().all(|n| *n < current_tree)
            || row[0..x].iter().all(|n| *n < current_tree)
            || row[x + 1..width].iter().all(|n| *n < current_tree)
            || column[y + 1..height].iter().all(|n| *n < current_tree)
    }

    pub fn count(&self) -> u64 {
        let mut count = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                if self.is_visible(x, y) {
                    count += 1;
                }
            }
        }

        count
    }
}

fn main() {
    let grid = Grid::new(include_str!("../input"));

    println!("{}", grid.count());
}
