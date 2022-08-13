use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug)]
struct Color([f64; 3]);

const IMAGE_WIDTH: u32 = 1000;
const IMAGE_HEIGHT: u32 = 1000;

fn main() {
    let mut pixels = Vec::<Color>::with_capacity(IMAGE_WIDTH as usize + IMAGE_HEIGHT as usize);
    for j in 0..IMAGE_HEIGHT {
        let mut line = (0..IMAGE_WIDTH)
            .into_iter()
            .map(|i| {
                Color([
                    i as f64 / IMAGE_WIDTH as f64,
                    j as f64 / IMAGE_HEIGHT as f64,
                    0.5,
                ])
            })
            .collect::<Vec<Color>>();
        pixels.append(&mut line);
    }

    save_ppm("render.ppm", &pixels).unwrap();
}

fn save_ppm<P: AsRef<Path>>(path: P, pixels: &[Color]) -> io::Result<()> {
    let mut file = File::create(path)?;
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_HEIGHT, IMAGE_WIDTH)?;
    writeln!(file, "255")?;
    for Color([r, g, b]) in pixels {
        let to255 = |x| (x * 255.99) as u8;
        writeln!(file, "{} {} {}", to255(r), to255(g), to255(b))?;
    }
    file.flush()?;

    Ok(())
}
