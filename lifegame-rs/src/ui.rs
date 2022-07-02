use std::fmt;

use crate::game::{Cell, Game};

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Cell::Dead => write!(f, ".."),
            Cell::Alive => write!(f, "##"),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in self.iter() {
            s += row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("")
                .as_str();
            s += "\n"
        }
        write!(f, "{}", &s[..s.len()-1])
    }
}
