fn main() -> Result<(), Box<dyn std::error::Error>> {
    let str2vec = |s: &str| {
        s.lines()
            .map(|num| num.parse::<u64>().unwrap())
            .collect::<Vec<u64>>()
    };

    let calories = include_str!("../input")
        .split("\n\n")
        .map(|s| str2vec(s))
        .map(|v| v.iter().sum::<u64>())
        .collect::<Vec<u64>>();

    let most = calories.iter().max().unwrap();

    println!("{:?}", most);

    Ok(())
}
