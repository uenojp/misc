fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str2vec = |s: &str| {
        s.lines()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    };

    let mut calories = include_str!("../input")
        .split("\n\n")
        .map(|s| str2vec(s))
        .map(|v| v.iter().sum::<u64>())
        .collect::<Vec<u64>>();

    calories.sort();
    calories.reverse();

    let top3sum = calories.iter().take(3).sum::<u64>();

    println!("{:?}", top3sum);

    Ok(())
}
