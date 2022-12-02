#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Loss,
    Draw,
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
            "A" | "X" => Some(Shape::Rock),
            "B" | "Y" => Some(Shape::Paper),
            "C" | "Z" => Some(Shape::Scissors),
            _ => None,
        }
    }

    pub fn play(&self, opponent: &Shape) -> Outcome {
        use Outcome::*;
        use Shape::*;

        match self {
            Rock => match opponent {
                Rock => Draw,
                Paper => Loss,
                Scissors => Win,
            },
            Paper => match opponent {
                Rock => Win,
                Paper => Draw,
                Scissors => Loss,
            },
            Scissors => match opponent {
                Rock => Loss,
                Paper => Win,
                Scissors => Draw,
            },
        }
    }
}

fn main() {
    let score = include_str!("../input")
        .lines()
        .map(|s| s.split_once(' ').unwrap())
        .map(|(opponent, my)| (Shape::new(opponent).unwrap(), Shape::new(my).unwrap()))
        .fold(0, |mut total, (opponent, my)| {
            total += match my.play(&opponent) {
                Outcome::Win => 6,
                Outcome::Draw => 3,
                Outcome::Loss => 0,
            };
            total += match my {
                Shape::Rock => 1,
                Shape::Paper => 2,
                Shape::Scissors => 3,
            };
            total
        });

    println!("{:?}", score);
}
