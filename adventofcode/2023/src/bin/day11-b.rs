use std::{
    collections::{HashSet, VecDeque},
    io::{self, BufRead},
};

struct Simulator {
    universe: Vec<Vec<char>>,
    galaxy_indices: Vec<(usize, usize)>,
    no_galaxy_row_indices: HashSet<usize>,
    no_galaxy_column_indices: HashSet<usize>,
}

impl Simulator {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let universe = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut galaxy_indices = vec![];
        for (height, row) in universe.iter().enumerate() {
            for (width, space) in row.iter().enumerate() {
                if *space != '.' {
                    galaxy_indices.push((height, width));
                }
            }
        }

        let mut no_galaxy_row_indices = HashSet::new();
        for (i, row) in universe.iter().enumerate() {
            if row.iter().all(|&c| c == '.') {
                no_galaxy_row_indices.insert(i);
            }
        }

        let mut no_galaxy_column_indices = HashSet::new();
        for width in 0..universe[0].len() {
            let mut no_galaxies = true;
            for height in 0..universe.len() {
                if universe[height][width] == '#' {
                    no_galaxies = false;
                    break;
                }
            }
            if no_galaxies {
                no_galaxy_column_indices.insert(width);
            }
        }

        Self {
            universe,
            galaxy_indices,
            no_galaxy_row_indices,
            no_galaxy_column_indices,
        }
    }

    fn bfs(&self, start: (usize, usize)) -> Vec<Vec<Option<usize>>> {
        // let gap = 2; // in day11-a
        // let gap = 10;
        // let gap = 100;
        let gap = 1000000;
        let mut distances = vec![vec![None; self.universe[0].len()]; self.universe.len()];
        let mut queue = VecDeque::new();
        distances[start.0][start.1] = Some(0);
        queue.push_back(start);
        while let Some(position) = queue.pop_front() {
            if position.0.checked_sub(1).is_some() {
                if distances[position.0 - 1][position.1].is_none() {
                    let dx = if self.no_galaxy_row_indices.contains(&(position.0 - 1)) {
                        gap
                    } else {
                        1
                    };
                    distances[position.0 - 1][position.1] =
                        distances[position.0][position.1].map(|d| d + dx);
                    queue.push_back((position.0 - 1, position.1));
                }
            }
            if position.1.checked_sub(1).is_some() {
                let dx = if self.no_galaxy_column_indices.contains(&(position.1 - 1)) {
                    gap
                } else {
                    1
                };
                if distances[position.0][position.1 - 1].is_none() {
                    distances[position.0][position.1 - 1] =
                        distances[position.0][position.1].map(|d| d + dx);
                    queue.push_back((position.0, position.1 - 1));
                }
            }
            if position.0 + 1 < self.universe.len() {
                let dx = if self.no_galaxy_row_indices.contains(&(position.0 + 1)) {
                    gap
                } else {
                    1
                };
                if distances[position.0 + 1][position.1].is_none() {
                    distances[position.0 + 1][position.1] =
                        distances[position.0][position.1].map(|d| d + dx);
                    queue.push_back((position.0 + 1, position.1));
                }
            }
            if position.1 + 1 < self.universe[0].len() {
                let dx = if self.no_galaxy_column_indices.contains(&(position.1 + 1)) {
                    gap
                } else {
                    1
                };
                if distances[position.0][position.1 + 1].is_none() {
                    distances[position.0][position.1 + 1] =
                        distances[position.0][position.1].map(|d| d + dx);
                    queue.push_back((position.0, position.1 + 1));
                }
            }
        }

        distances
    }

    fn sum_of_shortest_path(&self) -> usize {
        let mut sum = 0;
        for start in &self.galaxy_indices {
            let distances = self.bfs(*start);
            for another_galaxy in &self.galaxy_indices {
                sum += distances[another_galaxy.0][another_galaxy.1].unwrap();
            }
        }

        sum / 2
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let simulator = Simulator::parse(lines);

    println!("{}", simulator.sum_of_shortest_path());
}
