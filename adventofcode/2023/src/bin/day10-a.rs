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
    start: (usize, usize),
}

impl Sketch {
    fn parse(lines: impl Iterator<Item = String>) -> Self {
        let sketch = lines
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut start = None;
        for height in 0..sketch.len() {
            for width in 0..sketch[0].len() {
                if sketch[height][width] == 'S' {
                    start = Some((height, width));
                }
            }
        }

        Self {
            sketch,
            start: start.unwrap(),
        }
    }

    fn steps_to_farthest_point(&self) -> u64 {
        let mut farthest = 0;
        let mut distances = vec![vec![None; self.sketch[0].len()]; self.sketch.len()];
        let mut queue = VecDeque::new();

        distances[self.start.0][self.start.1] = Some(0);
        queue.push_back(self.start);

        while let Some(position) = queue.pop_front() {
            // | is a vertical pipe connecting north and south.
            // - is a horizontal pipe connecting east and west.
            // L is a 90-degree bend connecting north and east.
            // J is a 90-degree bend connecting north and west.
            // 7 is a 90-degree bend connecting south and west.
            // F is a 90-degree bend connecting south and east.
            // . is ground; there is no pipe in this tile.
            // S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.

            // debug(&distances);

            let current_distance =
                distances[position.0][position.1].expect("the distance must be determined");
            let current_pipe = self.sketch[position.0][position.1];

            let mut go_ahead = |next_position: (usize, usize),
                                current_pipes: &[char],
                                next_pipes: &[char]| {
                if current_pipes.contains(&current_pipe)
                    && next_pipes.contains(&self.sketch[next_position.0][next_position.1])
                {
                    if distances[next_position.0][next_position.1].is_none() {
                        distances[next_position.0][next_position.1] = Some(current_distance + 1);
                        farthest = farthest.max(current_distance + 1);
                        queue.push_back((next_position.0, next_position.1));
                    }
                }
            };

            // up
            if position.0.checked_sub(1).is_some() {
                go_ahead(
                    (position.0 - 1, position.1),
                    &['S', '|', 'L', 'J'],
                    &['|', '7', 'F'],
                )
            }
            // down
            if position.0 + 1 < self.sketch.len() {
                go_ahead(
                    (position.0 + 1, position.1),
                    &['S', '|', 'F', '7'],
                    &['|', 'L', 'J'],
                )
            }
            // left
            if position.1.checked_sub(1).is_some() {
                go_ahead(
                    (position.0, position.1 - 1),
                    &['S', '-', 'J', '7'],
                    &['-', 'L', 'F'],
                )
            }
            // right
            if position.1 + 1 < self.sketch[0].len() {
                go_ahead(
                    (position.0, position.1 + 1),
                    &['S', '-', 'L', 'F'],
                    &['-', 'J', '7'],
                )
            }
        }

        // debug(&distances);

        farthest
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let sketch = Sketch::parse(lines);
    println!("{}", sketch.steps_to_farthest_point());
}
