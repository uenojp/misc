fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plan = include_str!("../input")
        .lines()
        .map(|s| s.split_whitespace().collect::<Vec<&str>>())
        .map(|v| -> Result<(&str, u64), std::num::ParseIntError> {
            Ok((v[0], v[1].parse::<u64>()?))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut aim = 0u64;
    let mut horizontal_position = 0u64;
    let mut depth = 0u64;
    for (direction, value) in plan {
        match direction {
            "down" => aim += value,
            "up" => aim -= value,
            "forward" => {
                horizontal_position += value;
                depth += aim * value
            }
            _ => return Err("Unexpected direction".into()),
        };
    }

    println!("{:?}", horizontal_position * depth);

    Ok(())
}
