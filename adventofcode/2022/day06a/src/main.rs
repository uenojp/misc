fn main() {
    let buffer = include_str!("../input")
        .chars()
        .enumerate()
        .map(|(i, c)| (i + 1, c))
        .collect::<Vec<_>>();

    let start_of_packat = Iterator::skip_while(buffer.windows(4), |w| {
        w[0].1 == w[1].1
            || w[0].1 == w[2].1
            || w[0].1 == w[3].1
            || w[1].1 == w[2].1
            || w[1].1 == w[3].1
            || w[2].1 == w[3].1
    })
    .next()
    .unwrap();

    println!("{:?}", start_of_packat[3].0);
}
