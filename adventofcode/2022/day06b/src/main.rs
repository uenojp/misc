use std::collections::HashSet;

fn main() {
    let buffer = include_str!("../input")
        .chars()
        .enumerate()
        .map(|(i, c)| (i + 1, c))
        .collect::<Vec<_>>();

    let is_uniq = |slice: &[(usize, char)]| {
        let mut set = HashSet::new();

        for (_, c) in slice {
            if set.contains(c) {
                return false;
            } else {
                set.insert(*c);
            }
        }
        true
    };

    let start_of_packat = Iterator::skip_while(buffer.windows(14), |w| !is_uniq(w))
        .next()
        .unwrap();

    println!("{:?}", start_of_packat[13].0);
}
