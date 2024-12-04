use std::io::{self, BufRead};

fn count_row(line: &str) -> u32 {
    let mut count = 0;
    let mut i = 0;
    while let Some(xmas_index) = line[i..].find("XMAS") {
        i += xmas_index + "XMAS".len();
        count += 1;
        if i >= line.len() {
            break;
        }
    }
    count
}

fn count_diagonal(chars: &[Vec<char>]) -> u32 {
    let mut count = 0;
    let (width, height) = (chars[0].len(), chars.len());
    for h in 0..(height - 3) {
        for w in 0..(width - 3) {
            if chars[h][w] == 'X'
                && chars[h + 1][w + 1] == 'M'
                && chars[h + 2][w + 2] == 'A'
                && chars[h + 3][w + 3] == 'S'
            {
                count += 1;
            }
        }
    }
    count
}

fn main() {
    let lines = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();
    let mut transposed_lines = vec![String::with_capacity(lines.len()); lines[0].len()];
    for line in &lines {
        for (i, c) in line.chars().enumerate() {
            transposed_lines[i].push(c);
        }
    }

    let mut answer = 0;

    for line in &lines {
        let rev = line.chars().rev().collect::<String>();
        answer += count_row(&line) + count_row(&rev);
    }

    for transposed_line in &transposed_lines {
        let transposed_rev = transposed_line.chars().rev().collect::<String>();
        answer += count_row(&transposed_line) + count_row(&transposed_rev);
    }

    let chars = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let rev_chars = lines
        .iter()
        .map(|line| line.chars().rev().collect())
        .collect::<Vec<Vec<char>>>();
    let flipped_chars = lines
        .iter()
        .map(|line| line.chars().collect())
        .rev()
        .collect::<Vec<Vec<char>>>();
    let flipped_rev_chars = lines
        .iter()
        .map(|line| line.chars().rev().collect())
        .rev()
        .collect::<Vec<Vec<char>>>();
    answer += count_diagonal(&chars);
    answer += count_diagonal(&rev_chars);
    answer += count_diagonal(&flipped_chars);
    answer += count_diagonal(&flipped_rev_chars);

    println!("{answer}");
}
