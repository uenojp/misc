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

    fn viewing_distance<'a>(&self, iter: impl Iterator<Item = &'a u64>, current_tree: u64) -> u64 {
        let mut count = 0;
        for &tree in iter {
            if tree >= current_tree {
                count += 1;
                break;
            }
            count += 1;
        }

        count
    }

    fn scentic_score(&self, x: usize, y: usize) -> u64 {
        let height = self.grid.len();
        let width = self.grid[0].len();

        let current_tree = self.grid[y][x];

        let row = self.grid[y].clone();
        let column = (0..height).map(|j| self.grid[j][x]).collect::<Vec<_>>();

        self.viewing_distance(column[0..y].iter().rev(), current_tree)            // up
            * self.viewing_distance(row[0..x].iter().rev(), current_tree)         // left
            * self.viewing_distance(row[x + 1..width].iter(), current_tree)       // right
            * self.viewing_distance(column[y + 1..height].iter(), current_tree)   // down
    }

    pub fn max_scentic_score(&self) -> u64 {
        (0..self.grid[0].len())
            .flat_map(|x| (0..self.grid.len()).map(move |y| (x, y)))
            .map(|(x, y)| self.scentic_score(x, y))
            .max()
            .unwrap()
    }
}

fn main() {
    let grid = Grid::new(include_str!("../input"));

    println!("{}", grid.max_scentic_score());
}
