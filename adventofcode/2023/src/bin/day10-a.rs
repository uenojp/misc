use std::{
    collections::VecDeque,
    io::{self, BufRead},
};

#[allow(unused)]
fn debug(distances: &[Vec<Option<u64>>]) {
    let s = distances
        .iter()
        .map(|row| {
            row.iter()
                .map(|d| d.map_or(String::from("- "), |x| format!("{:2}", x.to_string())))
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    eprintln!("{s}\n");
}

struct Sketch {
    sketch: Vec<Vec<char>>,
}

impl Sketch {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let sketch = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self { sketch }
    }

    fn farthest_point_steps(&self) -> u64 {
        let mut distances = vec![vec![None; self.sketch[0].len()]; self.sketch.len()];

        let mut start = None;
        for height in 0..self.sketch.len() {
            for width in 0..self.sketch[0].len() {
                if self.sketch[height][width] == 'S' {
                    start = Some((height, width));
                }
            }
        }

        let start = start.unwrap();
        let mut farthest = 0;
        let mut queue = VecDeque::new();

        distances[start.0][start.1] = Some(0);
        queue.push_back(start);

        while let Some(position) = queue.pop_front() {
            // debug(&distances);

            // | is a vertical pipe connecting north and south.
            // - is a horizontal pipe connecting east and west.
            // L is a 90-degree bend connecting north and east.
            // J is a 90-degree bend connecting north and west.
            // 7 is a 90-degree bend connecting south and west.
            // F is a 90-degree bend connecting south and east.
            // . is ground; there is no pipe in this tile.
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

            let current_distance =
                distances[position.0][position.1].expect("the distance must be determined");
            let current_pipe = self.sketch[position.0][position.1];

            // up
            if position.0.checked_sub(1).is_some()
                && matches!(current_pipe, 'S' | '|' | 'L' | 'J')
                && matches!(self.sketch[position.0 - 1][position.1], '|' | '7' | 'F')
            {
                if distances[position.0 - 1][position.1].is_none() {
                    distances[position.0 - 1][position.1] = Some(current_distance + 1);
                    farthest = farthest.max(current_distance + 1);
                    queue.push_back((position.0 - 1, position.1));
                }
            }
            // down
            if position.0 + 1 < self.sketch.len()
                && matches!(current_pipe, 'S' | '|' | 'F' | '7')
                && matches!(self.sketch[position.0 + 1][position.1], '|' | 'L' | 'J')
            {
                if distances[position.0 + 1][position.1].is_none() {
                    distances[position.0 + 1][position.1] = Some(current_distance + 1);
                    farthest = farthest.max(current_distance + 1);
                    queue.push_back((position.0 + 1, position.1));
                }
            }
            // left
            if position.1.checked_sub(1).is_some()
                && matches!(current_pipe, 'S' | '-' | 'J' | '7')
                && matches!(self.sketch[position.0][position.1 - 1], '-' | 'L' | 'F')
            {
                if distances[position.0][position.1 - 1].is_none() {
                    distances[position.0][position.1 - 1] = Some(current_distance + 1);
                    farthest = farthest.max(current_distance + 1);
                    queue.push_back((position.0, position.1 - 1));
                }
            }
            // right
            if position.1 + 1 < self.sketch[0].len()
                && matches!(current_pipe, 'S' | '-' | 'L' | 'F')
                && matches!(self.sketch[position.0][position.1 + 1], '-' | 'J' | '7')
            {
                if distances[position.0][position.1 + 1].is_none() {
                    distances[position.0][position.1 + 1] = Some(current_distance + 1);
                    farthest = farthest.max(current_distance + 1);
                    queue.push_back((position.0, position.1 + 1));
                }
            }
        }

        // debug(&distances);

        farthest
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let sketch = Sketch::parse(lines);
    println!("{}", sketch.farthest_point_steps());
}
