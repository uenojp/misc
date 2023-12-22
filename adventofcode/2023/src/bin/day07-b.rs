use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Eq, Debug)]
struct HandBid {
    hand: String,
    bid: u32,
    frequency: HashMap<char, u32>,
    num_wildcards: u32,
}

impl HandBid {
    fn parse(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = hand.to_string();
        assert!(hand.len() == 5);
        let bid = bid.parse::<u32>().unwrap();

        let mut frequency = HashMap::new();
        let mut num_wildcards = 0;
        for c in hand.chars() {
            if c == 'J' {
                num_wildcards += 1;
            } else {
                *frequency.entry(c).or_insert(0) += 1;
            }
        }

        Self {
            hand,
            bid,
            frequency,
            num_wildcards,
        }
    }

    fn strength(&self) -> u32 {
        let (&max_card, &max_count) = self
            .frequency
            .iter()
            .max_by(|&(_, count1), &(_, count2)| count1.cmp(count2))
            .unwrap_or((&'-', &0));
        let mut max_count = max_count;
        max_count += self.num_wildcards;

        let second_count = self
            .frequency
            .iter()
            .filter(|(&card, _)| card != max_card)
            .map(|(_, &count)| count)
            .max()
            .unwrap_or(0);

        if max_count == 5 {
            6
        } else if max_count == 4 {
            5
        } else if max_count == 3 && second_count == 2 {
            4
        } else if max_count == 3 {
            3
        } else if max_count == 2 && second_count == 2 {
            2
        } else if max_count == 2 {
            1
        } else {
            0
        }
    }
}

//
// Ordering
//
impl Ord for HandBid {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        match self.strength().cmp(&other.strength()) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                let strength = |card| match card {
                    'A' => 12,
                    'K' => 11,
                    'Q' => 10,
                    'T' => 9,
                    '9' => 8,
                    '8' => 7,
                    '7' => 6,
                    '6' => 5,
                    '5' => 4,
                    '4' => 3,
                    '3' => 2,
                    '2' => 1,
                    'J' => 0,
                    _ => unreachable!(),
                };
                for (self_card, other_card) in self.hand.chars().zip(other.hand.chars()) {
                    match strength(self_card).cmp(&strength(other_card)) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => continue,
                        Ordering::Greater => return Ordering::Greater,
                    }
                }
                Ordering::Equal
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for HandBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HandBid {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);
    let mut hand_bids = lines.map(|line| HandBid::parse(&line)).collect::<Vec<_>>();
    hand_bids.sort();
    let total: usize = hand_bids
        .iter()
        .enumerate()
        .map(|(rank, hand_bid)| (rank + 1) * hand_bid.bid as usize)
        .sum();
    println!("{total}");
}
