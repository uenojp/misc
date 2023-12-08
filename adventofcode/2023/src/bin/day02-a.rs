use std::io::{self, BufRead};

#[derive(Debug)]
struct Game {}

impl Game {
    fn is_possible(line: &str) -> (u64, bool) {
        let (game_id, rounds) = line
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

        for round in rounds {
            if round
                .iter()
                .find(|&cube| match cube {
                    Cube::Red(amount) => *amount > 12,
                    Cube::Blue(amount) => *amount > 14,
                    Cube::Green(amount) => *amount > 13,
                })
                .is_some()
            {
                return (game_id, false);
            }
        }

        (game_id, true)
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

    let sum = lines
        .map(|line| Game::is_possible(&line))
        .filter(|(_, possible)| *possible)
        .map(|(game_id, _)| game_id)
        .sum::<u64>();
    println!("{}", sum);
}
