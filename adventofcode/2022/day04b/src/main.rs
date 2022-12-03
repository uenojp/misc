fn main() {
    let count = include_str!("../input")
        .lines()
        .map(|s| s.split_once(',').unwrap())
        .map(|(first, second)| {
            (
                first.split_once('-').unwrap(),
                second.split_once('-').unwrap(),
            )
        })
        .map(|((l1, r1), (l2, r2))| {
            (
                (l1.parse::<u64>().unwrap(), r1.parse::<u64>().unwrap()),
                (l2.parse::<u64>().unwrap(), r2.parse::<u64>().unwrap()),
            )
        })
        .filter(|((l1, r1), (l2, r2))| !(r1 < l2 || r2 < l1))
        .count();

    println!("{count:?}");
}
