use rayt::{Color, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

#[derive(Debug, Clone)]
pub struct ColorTexture {
    color: Color,
}

impl ColorTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for ColorTexture {
    fn value(&self, _: f64, _: f64, _: Point3) -> Color {
        self.color
    }
}
