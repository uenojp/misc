mod exponential;
mod liner;
mod logn;
mod mat2;

use std::time::Instant;

fn main() {
    let n = 40;

    let start = Instant::now();
    let result = exponential::fibonacci(n);
    let duration = start.elapsed();
    println!(
        "{}...\n{:?}",
        result.to_string().chars().take(10).collect::<String>(),
        duration
    );

    let n = 1000000;

    let start = Instant::now();
    let result = liner::fibonacci(n);
    let duration = start.elapsed();
    println!(
        "{}...\n{:?}",
        result.to_string().chars().take(10).collect::<String>(),
        duration
    );

    let start = Instant::now();
    let result = logn::fibonacci(n);
    let duration = start.elapsed();
    println!(
        "{}...\n{:?}",
        result.to_string().chars().take(10).collect::<String>(),
        duration
    );
}
