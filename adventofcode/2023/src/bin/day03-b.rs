use std::{
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let schematic = io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut coord_number = HashMap::new();
    let mut number_coords: HashMap<u32, Vec<Vec<(usize, usize)>>> = HashMap::new();

    for (i, line) in schematic.iter().enumerate() {
        let mut num = None;
        let mut coords = vec![];
        for (j, c) in line.iter().enumerate() {
            if c.is_ascii_digit() {
                num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap());
                coords.push((i, j));
            } else {
                if let Some(n) = num {
                    for (x, y) in &coords {
                        coord_number.insert((*x, *y), n);
                    }
                    number_coords
                        .entry(n)
                        .and_modify(|cds| cds.push(coords.clone()))
                        .or_insert(vec![coords.clone()]);
                }
                coords.clear();
                num = None;
            }
        }

        // care number at end like '....123'
        if let Some(n) = num {
            for (x, y) in &coords {
                coord_number.insert((*x, *y), n);
            }
            number_coords
                .entry(n)
                .and_modify(|cds| cds.push(coords.clone()))
                .or_insert(vec![coords.clone()]);
        }
    }

    // eprintln!("{:?}", coord_number);
    // eprintln!("---");
    // eprintln!("{:?}", number_coords);
    // eprintln!("====");

    let mut sum = 0;
    let (height, width) = (schematic.len(), schematic[0].len());
    for (i, line) in schematic.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            // if !"#%&*+-/=@$".contains(*c) {
            if *c != '*' {
                continue;
            }

            let mut gear_parts = vec![];
            for di in -1..=1 {
                for dj in -1..=1 {
                    let next = (i as isize + di, j as isize + dj);
                    if next.0 < 0
                        || next.0 >= height as isize
                        || next.1 < 0
                        || next.1 >= width as isize
                    {
                        continue;
                    }

                    let next = (next.0 as usize, next.1 as usize);
                    if let Some(num) = coord_number.remove(&next) {
                        // eprintln!("({i},{j}) ({},{}) {c} : {num}", next.0, next.1);
                        for used_coords_candidate in number_coords.get(&num).unwrap() {
                            if used_coords_candidate.contains(&next) {
                                for used_cord in used_coords_candidate {
                                    coord_number.remove(used_cord);
                                }
                            }
                        }
                        gear_parts.push(num);
                    }
                }
            }

            if gear_parts.len() == 2{
                sum += gear_parts[0] * gear_parts[1];
            }
        }
    }
    println!("{sum}");
}
