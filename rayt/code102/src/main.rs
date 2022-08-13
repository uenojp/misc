use rayt::{self, Camera, Color, Ray, Vec3};

use image::{Rgb, RgbImage};
use rayon::prelude::*;

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1500;

fn color(ray: &Ray) -> Color {
    let direction = ray.direction.normalize();
    let t = 0.5 * (direction.y() + 1.0);
    Color::new(0.2, 0.2, 1.0).lerp(Color::one(), t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Camera::new(
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
    );

    // カメラは原点
    // 対象物はカメラから10離れた場所
    // カメラの傾きは(0.5, 1.0, 0.0)、つまり右に45度傾く
    // 視野角は60度
    // アスペクト比は1500/1500=1.0
    let camera = Camera::from_lookat(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, -10.0),
        Vec3::new(0.5, 1.0, 0.0).normalize(),
        60.0,
        1.0,
    );

    let mut image = RgbImage::new(WIDTH, HEIGHT);
    image
        .enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f64 / (WIDTH - 1) as f64;
            let v = *y as f64 / (HEIGHT - 1) as f64;
            let ray = camera.ray(u, v);
            let rgb = color(&ray).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    image.save("render.png")?;
    Ok(())
}
