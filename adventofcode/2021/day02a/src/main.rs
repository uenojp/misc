fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plan = include_str!("../input")
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| -> Result<(&str, u64), std::num::ParseIntError> {
            Ok((v[0], v[1].parse::<u64>()?))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let position = |direction: &str| {
        plan.iter()
            .filter(|(dir, _)| *dir == direction)
            .map(|(_, value)| value)
            .sum::<u64>()
    };

    println!(
        "{:?}",
        position("forward") * (position("down") - position("up"))
    );

    Ok(())
}
