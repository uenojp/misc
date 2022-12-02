#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    pub fn new(s: &str) -> Option<Self> {
        match s {
            "X" => Some(Outcome::Loss),
            "Y" => Some(Outcome::Draw),
            "Z" => Some(Outcome::Win),
            _ => None,
        }
    }

    pub fn my_hand(&self, opponent: &Shape) -> Shape {
        use Outcome::*;
        use Shape::*;

        match self {
            Win => match opponent {
                Rock => Paper,
                Paper => Scissors,
                Scissors => Rock,
            },
            Loss => match opponent {
                Rock => Scissors,
                Paper => Rock,
                Scissors => Paper,
            },
            Draw => match opponent {
                Rock => Rock,
                Paper => Paper,
                Scissors => Scissors,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn new(s: &str) -> Option<Self> {
        match s {
            "A" => Some(Shape::Rock),
            "B" => Some(Shape::Paper),
            "C" => Some(Shape::Scissors),
            _ => None,
        }
    }
}

fn main() {
    let score = include_str!("../input")
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(opponent, outcome)| (Shape::new(opponent).unwrap(), Outcome::new(outcome).unwrap()))
        .fold(0, |mut total, (opponent, outcome)| {
            total += match outcome {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            };
            total += match outcome.my_hand(&opponent) {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            total
        });

    println!("{:?}", score);
}
