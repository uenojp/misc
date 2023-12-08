use std::io::{self, BufRead};

#[derive(Debug)]
struct Game {}

impl Game {
    fn power(line: &str) -> u64 {
        let (_, rounds) = line
            .split_once(':')
            .map(|(game, rounds)| {
                let (game, game_id) = game.split_once(' ').unwrap();
                assert_eq!(game, "Game");

                let rounds = rounds
                    .split(';')
                    .map(|round| {
                        round
                            .split(',')
                            .map(|round| Cube::parse(round.trim()))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();

                (game_id.parse::<u64>().unwrap(), rounds)
            })
            .unwrap();

        let mut max = (0, 0, 0);
        // eprintln!("{:?}", rounds);
        for round in rounds {
            for cube in round {
                match cube {
                    Cube::Red(amont) => max = (max.0.max(amont), max.1, max.2),
                    Cube::Blue(amont) => max = (max.0, max.1.max(amont), max.2),
                    Cube::Green(amont) => max = (max.0, max.1, max.2.max(amont)),
                }
            }
        }
        // eprintln!("{:?}", max);

        max.0 * max.1 * max.2
    }
}

#[derive(Debug)]
enum Cube {
    Red(u64),
    Blue(u64),
    Green(u64),
}

impl Cube {
    fn parse(cube: &str) -> Self {
        let (amount, color) = cube.split_once(' ').unwrap();
        let amount = amount.parse::<u64>().unwrap();
        match color {
            "red" => Self::Red(amount),
            "blue" => Self::Blue(amount),
            "green" => Self::Green(amount),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let sum = lines.map(|line| Game::power(&line)).sum::<u64>();
    println!("{}", sum);
}
