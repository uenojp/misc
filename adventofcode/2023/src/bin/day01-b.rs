use proconio::{input, is_stdin_empty};

fn find(s: &str, chars: &[char], i: usize) -> Option<u32> {
    if chars[i].is_ascii_digit() {
        chars[i].to_digit(10)
    } else if s[i..].starts_with("one") {
        Some(1)
    } else if s[i..].starts_with("two") {
        Some(2)
    } else if s[i..].starts_with("three") {
        Some(3)
    } else if s[i..].starts_with("four") {
        Some(4)
    } else if s[i..].starts_with("five") {
        Some(5)
    } else if s[i..].starts_with("six") {
        Some(6)
    } else if s[i..].starts_with("seven") {
        Some(7)
    } else if s[i..].starts_with("eight") {
        Some(8)
    } else if s[i..].starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn main() {
    let mut answer = 0;

    loop {
        if is_stdin_empty() {
            break;
        }

        input! { s: String }
        let chars = s.chars().collect::<Vec<_>>();

        let mut first_digit = None;
        let mut last_digit = None;

        for i in 0..s.len() {
            first_digit = find(&s, &chars, i);
            if first_digit.is_some() {
                break;
            }
        }

        for i in (0..s.len()).rev() {
            last_digit = find(&s, &chars, i);
            if last_digit.is_some() {
                break;
            }
        }

        answer += first_digit.unwrap() * 10 + last_digit.unwrap();
    }

    println!("{answer}");
}
