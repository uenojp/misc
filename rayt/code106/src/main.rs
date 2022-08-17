use rayt::{render, Camera, Color, Point3, Ray, Scene, Vec3};

struct SimpleScene;

impl SimpleScene {
    fn hit_sphere(&self, center: Point3, radius: f64, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - radius.powi(2);
        let discriminant = b * b - a * c;
        if 0.0 <= discriminant {
            Some((-b - discriminant) / a)
        } else {
            None
        }
    }
}

impl Scene for SimpleScene {
    fn camera(&self) -> Camera {
        Camera::new(
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: &rayt::Ray) -> Color {
        let center = Point3::new(0.0, 0.0, -1.0);
        let radius = 0.5;
        match self.hit_sphere(center, radius, ray) {
            Some(t) => {
                let normal = (ray.at(t) - center).normalize();
                0.5 * (normal + Color::one())
            }
            None => Color::new(0.0, 0.0, 0.5 * (ray.direction.normalize().y() + 1.0)),
        }
    }

    fn width(&self) -> u32 {
        720
    }

    fn height(&self) -> u32 {
        360
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render("render.png", SimpleScene {})?;
    Ok(())
}
