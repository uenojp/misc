use std::io::{self, BufRead};

#[derive(Debug)]
struct Card {
    #[allow(dead_code)]
    id: u64,
    winning_numbers: Vec<u64>,
    my_numbers: Vec<u64>,
}

impl Card {
    fn parse(line: &str) -> Self {
        let (id, winning_numbers, my_numbers) = line
            .split_once(':')
            .map(|(card, numbers)| {
                let mut parts = card.split_whitespace();
                let prefix = parts.next().unwrap();
                let id = parts.next().unwrap().parse::<u64>().unwrap();
                assert_eq!(prefix, "Card");

                let (winning_numbers, my_numbers) = numbers
                    .split_once('|')
                    .map(|(winning, my)| {
                        let winning = winning
                            .trim()
                            .split_whitespace()
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        let my = my
                            .trim()
                            .split_whitespace()
                            .map(|n| n.parse::<u64>().unwrap())
                            .collect::<Vec<_>>();
                        (winning, my)
                    })
                    .unwrap();

                (id, winning_numbers, my_numbers)
            })
            .unwrap();

        Self {
            id,
            winning_numbers,
            my_numbers,
        }
    }

    fn calculate_points(&self) -> u64 {
        let mut matches = 0u32;
        for winning_number in &self.winning_numbers {
            if self.my_numbers.contains(&winning_number) {
                matches += 1;
            }
        }
        if matches == 0 {
            0
        } else {
            2u64.pow(matches - 1)
        }
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let sum: u64 = lines
        .map(|line| Card::parse(&line))
        .map(|card| card.calculate_points())
        .sum();

    println!("{}", sum);
}
