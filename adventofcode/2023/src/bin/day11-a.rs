use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

fn bfs(expanded_universe: &[Vec<char>], start: (usize, usize)) -> Vec<Vec<Option<usize>>> {
    let mut distances = vec![vec![None; expanded_universe[0].len()]; expanded_universe.len()];
    let mut queue = VecDeque::new();
    distances[start.0][start.1] = Some(0);
    queue.push_back(start);
    while let Some(position) = queue.pop_front() {
        if position.0.checked_sub(1).is_some() {
            if distances[position.0 - 1][position.1].is_none() {
                distances[position.0 - 1][position.1] =
                    distances[position.0][position.1].map(|d| d + 1);
                queue.push_back((position.0 - 1, position.1));
            }
        }
        if position.1.checked_sub(1).is_some() {
            if distances[position.0][position.1 - 1].is_none() {
                distances[position.0][position.1 - 1] =
                    distances[position.0][position.1].map(|d| d + 1);
                queue.push_back((position.0, position.1 - 1));
            }
        }
        if position.0 + 1 < expanded_universe.len() {
            if distances[position.0 + 1][position.1].is_none() {
                distances[position.0 + 1][position.1] =
                    distances[position.0][position.1].map(|d| d + 1);
                queue.push_back((position.0 + 1, position.1));
            }
        }
        if position.1 + 1 < expanded_universe[0].len() {
            if distances[position.0][position.1 + 1].is_none() {
                distances[position.0][position.1 + 1] =
                    distances[position.0][position.1].map(|d| d + 1);
                queue.push_back((position.0, position.1 + 1));
            }
        }
    }

    distances
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let mut expanded_universe = vec![];
    for line in lines {
        let chars = line.chars().collect::<Vec<_>>();
        if chars.iter().all(|&c| c == '.') {
            expanded_universe.push(chars.clone());
        }
        expanded_universe.push(chars);
    }
    for width in (0..expanded_universe[0].len()).rev() {
        let mut no_galaxies = true;
        for height in 0..expanded_universe.len() {
            if expanded_universe[height][width] == '#' {
                no_galaxies = false;
                break;
            }
        }

        if no_galaxies {
            for row in expanded_universe.iter_mut() {
                row.insert(width, '.');
            }
        }
    }

    let mut galaxy_indices = vec![];
    for (height, row) in expanded_universe.iter().enumerate() {
        for (width, space) in row.iter().enumerate() {
            if *space != '.' {
                galaxy_indices.push((height, width));
            }
        }
    }

    let mut sum = 0;
    for start in &galaxy_indices {
        let distances = bfs(&expanded_universe, *start);
        for another_galaxy in &galaxy_indices {
            sum += distances[another_galaxy.0][another_galaxy.1].unwrap();
        }
    }
    println!("{}", sum / 2);
}
