use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
pub struct Command {
    pub direction: char,
    pub steps: i32,
}

impl Command {
    pub fn new(s: &str) -> Self {
        let (direction, steps) = s.split_once(' ').unwrap();
        Self {
            direction: direction.chars().next().unwrap(),
            steps: steps.parse().unwrap(),
        }
    }
}

pub struct Simulator {
    width: i32,
    height: i32,
    head: (i32, i32),
    tail: (i32, i32),
    tail_visited: HashSet<(i32, i32)>,
}

impl Simulator {
    // Width and height are used only for display.
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            head: (0, 0),
            tail: (0, 0),
            tail_visited: HashSet::new(),
        }
    }

    pub fn apply(&mut self, command: Command) {
        for _ in 0..command.steps {
            self.step(command.direction);
            self.tail_visited.insert(self.tail);
            // println!("{:?} {:?}", self.head, self.tail);
            // println!("{}", self);
        }
    }

    fn step(&mut self, direction: char) {
        let next_head = match direction {
            'U' => (self.head.0, self.head.1 + 1),
            'D' => (self.head.0, self.head.1 - 1),
            'L' => (self.head.0 - 1, self.head.1),
            'R' => (self.head.0 + 1, self.head.1),
            _ => unreachable!(),
        };

        let next_tail =
            if (next_head.0 - self.tail.0).abs() <= 1 && (next_head.1 - self.tail.1).abs() <= 1 {
                self.tail
            } else {
                self.head
            };

        self.head = next_head;
        self.tail = next_tail;
    }
}

impl fmt::Display for Simulator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();

        for y in 0..self.height {
            for x in 0..self.width {
                s += if self.head.0 == x && self.head.1 == y {
                    "H"
                } else if self.tail.0 == x && self.tail.1 == y {
                    "T"
                } else if x == 0 && y == 0 {
                    "s"
                } else {
                    "."
                };
            }
            s += "\n";
        }
        write!(f, "{}", s.split_inclusive('\n').rev().collect::<String>())
    }
}

fn main() {
    let commands = include_str!("../input")
        .lines()
        .map(Command::new)
        .collect::<Vec<_>>();

    // Width and height are used only for display.
    // 6x5 is for sample.
    let mut simulator = Simulator::new(6, 5);

    for command in commands {
        simulator.apply(command);
    }

    println!("{:?}", simulator.tail_visited.len());
}
