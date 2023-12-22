use std::{
    collections::HashMap,
    io::{self, BufRead},
};

#[derive(Eq, Debug)]
struct HandBid {
    hand: String,
    bid: u32,
    frequency: HashMap<char, u32>,
}

impl HandBid {
    fn parse(line: &str) -> Self {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = hand.to_string();
        assert!(hand.len() == 5);
        let bid = bid.parse::<u32>().unwrap();

        let mut frequency = HashMap::new();
        for c in hand.chars() {
            *frequency.entry(c).or_insert(0) += 1;
        }

        Self {
            hand,
            bid,
            frequency,
        }
    }

    fn strength(&self) -> u32 {
        if self.frequency.iter().any(|(_, &count)| count == 5) {
            6
        } else if self.frequency.iter().any(|(_, &count)| count == 4) {
            5
        } else if self.frequency.iter().any(|(_, &count)| count == 3)
            && self.frequency.iter().any(|(_, &count)| count == 2)
        {
            4
        } else if self.frequency.iter().any(|(_, &count)| count == 3) {
            3
        } else if self
            .frequency
            .iter()
            .filter(|(_, &count)| count == 2)
            .count()
            == 2
        {
            2
        } else if self.frequency.iter().any(|(_, &count)| count == 2) {
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
                    'J' => 9,
                    'T' => 8,
                    '9' => 7,
                    '8' => 6,
                    '7' => 5,
                    '6' => 4,
                    '5' => 3,
                    '4' => 2,
                    '3' => 1,
                    '2' => 0,
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
