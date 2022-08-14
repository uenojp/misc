use rayt::{Camera, Color, Point3, Ray, Vec3};

use image::{Rgb, RgbImage};
use rayon::prelude::*;

const WIDTH: u32 = 1000;
const HEIGHT: u32 = 500;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> bool {
    let oc = ray.origin - center;
    let discriminant = oc.dot(ray.direction).powi(2)
        - (ray.direction.dot(ray.direction)) * (oc.dot(oc) - radius.powi(2));
    0.0 <= discriminant
}

fn color(ray: &Ray) -> Color {
    let center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    let direction = ray.direction.normalize();
    let t = 0.25 * (direction.y() + 1.0).powi(2);
    if hit_sphere(center, radius, ray) {
        return Color::new(1.0, 0.0, 0.0).lerp(Color::one(), t);
    }

    let t = 0.5 * (direction.y() + 1.0);
    Color::new(0.2, 0.2, 1.0).lerp(Color::one(), t)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Camera::new(
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
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
