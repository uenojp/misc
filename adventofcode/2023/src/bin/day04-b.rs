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

    fn matches(&self) -> u64 {
        let mut matches = 0;
        for winning_number in &self.winning_numbers {
            if self.my_numbers.contains(&winning_number) {
                matches += 1;
            }
        }
        matches
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let matches = lines
        .map(|line| Card::parse(&line))
        .map(|card| card.matches())
        .collect::<Vec<_>>();
    let mut num_cards = vec![1; matches.len()];
    for i in 0..num_cards.len() {
        for j in 1..=matches[i] as usize {
            if i + j < num_cards.len() {
                num_cards[i + j] += num_cards[i];
            }
        }
    }
    // eprintln!("{:?}", cards);

    println!("{}", num_cards.iter().sum::<u64>());
}
