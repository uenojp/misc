use proconio::{input, is_stdin_empty};

fn main() {
    let mut answer = 0;

    loop {
        if is_stdin_empty() {
            break;
        }

        input! { s: String }

        let first_digit = s.chars().find(char::is_ascii_digit).unwrap();
        let last_digit = s.chars().rev().find(char::is_ascii_digit).unwrap();

        answer += first_digit.to_digit(10).unwrap() * 10 + last_digit.to_digit(10).unwrap();
    }

    println!("{answer}");
}
