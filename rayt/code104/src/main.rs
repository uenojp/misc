use image::{Rgb, RgbImage};
use rayon::prelude::*;
use rayt::{Camera, Color, Point3, Ray, Vec3};

const WIDTH: u32 = 720;
const HEIGHT: u32 = 360;

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> Option<f64> {
    let oc = ray.origin - center;
    let a = ray.direction.dot(ray.direction);
    let b = ray.direction.dot(oc);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b * b - a * c;
    if 0.0 <= discriminant {
        Some((-b - discriminant.sqrt()) / a)
    } else {
        None
    }
}

fn color(ray: &Ray) -> Color {
    let center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    match hit_sphere(center, radius, ray) {
        Some(t) => {
            let normal = (ray.at(t) - center).normalize();
            0.5 * (normal + Color::one())
        }
        None => Color::new(0.0, 0.0, 0.5 * (ray.direction.normalize().y() + 1.0)),
    }
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
        .into_iter()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f64 / (WIDTH - 1) as f64;
            let v = (HEIGHT - 1 - *y) as f64 / (HEIGHT - 1) as f64;
            let ray = camera.ray(u, v);
            let rgb = color(&ray).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    image.save("render.png")?;

    Ok(())
}
