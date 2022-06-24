mod game;
mod pattern;
mod ui;

use std::thread;
use std::time::Duration;

use rand::Rng;

use game::{Cell, Game};

fn random_pattern(height: usize, width: usize) -> Vec<Vec<Cell>> {
    let mut pattern = vec![vec![Cell::Dead; width]; height];
    let mut rng = rand::thread_rng();
    for row in pattern.iter_mut() {
        for cell in row {
            *cell = if rng.gen_range(0..=1) == 0 {
                Cell::Dead
            } else {
                Cell::Alive
            };
        }
    }
    pattern
}

fn main() {
    let mut game = Game::new(53, 105);
    let (height, width) = game.size();

    game.place_pattern(&random_pattern(height, width), 0, 0);
    game.place_pattern(pattern::GLIDER_GUN, 0, 0);
    game.place_rotated_pattern(pattern::GLIDER_GUN, height/2, width/4, game::Rotate::R180);

    loop {
        println!("{}", game);
        thread::sleep(Duration::from_millis(100));
        print!("\x1b[{}A", height + 1);
        game.next();
    }
}
