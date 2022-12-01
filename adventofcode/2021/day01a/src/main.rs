fn main() -> Result<(), Box<dyn std::error::Error>> {
    let measurements = include_str!("../input")
        .lines()
        .map(|s| s.parse::<u64>())
        .collect::<Result<Vec<u64>, _>>()?;

    let count = measurements.windows(2).filter(|w| w[0] < w[1]).count();

    println!("{:?}", count);

    Ok(())
}
