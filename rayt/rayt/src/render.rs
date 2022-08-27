use std::path::Path;

use crate::{Camera, Color, Ray, Point3};

use image::{ImageResult, Rgb, RgbImage};
use rayon::prelude::*;

const SAMPLE_PER_PIXEL: usize = 100;

pub trait Scene {
    fn camera(&self) -> Camera;
    fn trace(&self, ray: &Ray) -> Color;
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn spp(&self) -> usize {
        SAMPLE_PER_PIXEL
    }
    fn aspect(&self) -> f64 {
        self.width() as f64 / self.height() as f64
    }
}

pub fn render<S: Scene + Sync, P: AsRef<Path>>(path: P, scene: S) -> ImageResult<()> {
    let camera = scene.camera();
    let mut image = RgbImage::new(scene.width(), scene.height());
    image
        .enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let u = *x as f64 / (scene.width() - 1) as f64;
            let v = (scene.height() - 1 - *y) as f64 / (scene.height() - 1) as f64;
            let ray = camera.ray(u, v);
            let rgb = scene.trace(&ray).to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    image.save(path)?;
    Ok(())
}

pub fn render_aa<S: Scene + Sync, P: AsRef<Path>>(path: P, scene: S) -> ImageResult<()> {
    let camera = scene.camera();
    let mut image = RgbImage::new(scene.width(), scene.height());
    image
        .enumerate_pixels_mut()
        .collect::<Vec<(u32, u32, &mut Rgb<u8>)>>()
        .par_iter_mut()
        .for_each(|(x, y, pixel)| {
            let mut color = Color::zero();
            for _ in 0..scene.spp() {
                let [dx, dy, _] = Point3::random().to_array();
                let u = (*x as f64 + dx) / (scene.width() - 1) as f64;
                let v = ((scene.height() - 1 - *y) as f64 + dy) / (scene.height() - 1) as f64;
                let ray = camera.ray(u, v);
                color += scene.trace(&ray);
            }
            color /= scene.spp() as f64;
            let rgb = color.to_rgb();
            pixel[0] = rgb[0];
            pixel[1] = rgb[1];
            pixel[2] = rgb[2];
        });
    image.save(path)?;
    Ok(())
}
