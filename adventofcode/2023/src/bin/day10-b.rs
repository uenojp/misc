use std::{
    any::TypeId,
    collections::VecDeque,
    fmt::Display,
    io::{self, BufRead},
};

#[allow(unused)]
fn debug<T: 'static + Display + Copy>(sketch: &[Vec<Option<T>>]) {
    let is_char_type = TypeId::of::<T>() == TypeId::of::<char>();
    let s = sketch
        .iter()
        .map(|row| {
            row.iter()
                .map(|d| {
                    if is_char_type {
                        d.map_or(format!("{:1}", '.'), |x| format!("{:1}", x.to_string()))
                    } else {
                        d.map_or(format!("{:3}", '.'), |x| format!("{:3}", x.to_string()))
                    }
                })
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

    fn count_enclosed_tiles(&self) -> u64 {
        let mut farthest = 0;
        let mut distances = vec![vec![None; self.sketch[0].len()]; self.sketch.len()];
        let mut visualized_sketch = vec![vec![None; self.sketch[0].len()]; self.sketch.len()];
        let mut queue = VecDeque::new();

        distances[self.start.0][self.start.1] = Some(0);
        // for debugging
        visualized_sketch[self.start.0][self.start.1] = Some('S');
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

            debug(&distances);
            debug(&visualized_sketch);

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
                        // for debugging
                        let c = match self.sketch[next_position.0][next_position.1] {
                            '|' => '┃',
                            '-' => '━',
                            'L' => '┗',
                            'J' => '┛',
                            '7' => '┓',
                            'F' => '┏',
                            _ => '.',
                        };
                        visualized_sketch[next_position.0][next_position.1] = Some(c);
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

        debug(&distances);
        debug(&visualized_sketch);

        // let mtu left_side_is_inside = None;
        let mut head_direction = self
            .around(self.start)
            .find(|next| distances[next.0][next.1].is_some_and(|d| d == 1))
            .map(|next| {
                (
                    next.0 as isize - self.start.0 as isize,
                    next.1 as isize - self.start.1 as isize,
                )
            })
            .unwrap();
        dbg!(head_direction);
        let mut enclosed_tiles = distances
            .iter()
            .map(|row| {
                row.iter()
                    // Nones are candidates of enclosed tiles
                    .map(|tile| tile.map_or(true, |_| false))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut current_distance = 0;
        let mut num_enclosed_tiles_left = 0;
        let mut num_enclosed_tiles_right = 0;

        while current_distance <= 2 * farthest {
            // FIXME:
            //
            // 0 1
            // 1 2 // the 1 dont know which 2 to choose next
            // 2
            head_direction = self
                .around(self.start)
                .find(|next| distances[next.0][next.1].is_some_and(|d| d == current_distance + 1))
                .map(|next| {
                    (
                        next.0 as isize - self.start.0 as isize,
                        next.1 as isize - self.start.1 as isize,
                    )
                })
                .unwrap();
            current_distance += 1;

            let left_direction = (-head_direction.1, head_direction.0);
            let right_direction = (head_direction.1, -head_direction.0);

            // TODO
        }

        0
    }

    fn around(&self, position: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let up = position
            .0
            .checked_sub(1)
            .into_iter()
            .map(move |_| (position.0 - 1, position.1));

        let down = (position.0 + 1 < self.sketch.len())
            .then(|| (position.0 + 1, position.1))
            .into_iter();

        let left = position
            .1
            .checked_sub(1)
            .into_iter()
            .map(move |_| (position.0, position.1 - 1));

        let right = (position.1 + 1 < self.sketch[0].len())
            .then(|| (position.0, position.1 + 1))
            .into_iter();

        up.chain(down).chain(left).chain(right)
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let sketch = Sketch::parse(lines);
    println!("{}", sketch.count_enclosed_tiles());
}
