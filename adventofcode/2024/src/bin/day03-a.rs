use std::io::{self, BufRead};

fn main() {
    let lines = io::stdin().lock().lines().map(Result::unwrap);

    let mut answer = 0;
    for line in lines {
        let mut i = 0;
        while i < line.len() {
            if line[i..].starts_with("mul(") {
                i += "mul(".len();
                let Some(comma_index) = line[i..].find(',') else {
                    break;
                };
                if let Ok(lhs) = line[i..i + comma_index].parse::<u32>() {
                    i += comma_index + 1;
                    let Some(closing_index) = line[i..].find(')') else {
                        break;
                    };
                    if let Ok(rhs) = line[i..i + closing_index].parse::<u32>() {
                        answer += lhs * rhs;
                        continue;
                    }
                }
            }

            i += 1;
        }
    }

    println!("{answer}");
}
