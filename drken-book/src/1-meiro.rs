// 章末1.5
use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    input! {
        height: usize,
        width: usize,
        grid: [Chars; height],
    }

    dbg!(height, width, &grid);

    // Preprocessing
    let mut start = None;
    let mut goal = None;
    for (h, row) in grid.iter().enumerate() {
        for (w, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = Some((h, w));
            }
            if cell == 'G' {
                goal = Some((h, w));
            }
        }
    }
    let start = start.unwrap();
    let goal = goal.unwrap();

    // DFS
    let mut distance: Vec<Vec<Option<usize>>> = vec![vec![None; width]; height];
    distance[start.0][start.1] = Some(0);

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(current) = queue.pop_front() {
        for direction in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
            let next = (
                current.0 as isize + direction.0,
                current.1 as isize + direction.1,
            );
            if !(0 <= next.0 && next.0 < height as isize && 0 <= next.1 && next.1 < width as isize)
            {
                continue;
            }
            if grid[next.0 as usize][next.1 as usize] == '#' {
                continue;
            }

            if distance[next.0 as usize][next.1 as usize].is_none() {
                distance[next.0 as usize][next.1 as usize] =
                    Some(distance[current.0][current.1].unwrap() + 1);
                queue.push_back((next.0 as usize, next.1 as usize));
            }
        }
    }

    dbg!(&distance);
    dbg!(distance[goal.0][goal.1]);

    // Output
    for h in 0..height {
        for w in 0..width {
            if grid[h][w] == 'S' || grid[h][w] == '#' {
                print!("{:>3}", grid[h][w]);
            } else if let Some(dist) = distance[h][w] {
                print!("{:>3}", dist);
            } else {
                print!("{:>3}", '.');
            }
        }
        println!();
    }

    Ok(())
}
