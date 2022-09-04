mod material;
mod shape;
mod texture;

use rayt::{render_aa_width_depth, Camera, Color, Float3, Point3, Ray, SceneWithDepth, Vec3};

use material::{Dielectric, Lambertian, Material, Metal};
use shape::{HitInfo, Shape, Sphere};
use texture::{ColorTexture, Texture};

struct ShapeList {
    objects: Vec<Box<dyn Shape>>,
}

impl ShapeList {
    const fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn push(&mut self, object: Box<dyn Shape>) {
        self.objects.push(object);
    }
}

impl Shape for ShapeList {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let mut hit_info = None;
        let mut closest = t1;
        for object in &self.objects {
            if let Some(info) = object.hit(ray, t0, closest) {
                closest = info.t;
                hit_info = Some(info)
            }
        }
        hit_info
    }
}

struct ShapeBuilder<M: Material, T: Texture> {
    shape: Option<Box<dyn Shape>>,
    material: Option<M>,
    texture: Option<T>,
}

impl<M: Material + 'static, T: Texture + 'static> ShapeBuilder<M, T> {
    fn new() -> Self {
        Self {
            shape: None,
            material: None,
            texture: None,
        }
    }

    fn sphere(mut self, center: Point3, radius: f64) -> Self {
        self.shape = Some(Box::new(Sphere::new(
            center,
            radius,
            self.material.unwrap(),
        )));
        self.material = None;
        self
    }

    fn build(self) -> Box<dyn Shape> {
        self.shape.unwrap()
    }
}

impl ShapeBuilder<Lambertian<ColorTexture>, ColorTexture> {
    fn lambertian(mut self) -> Self {
        self.material = Some(Lambertian::new(self.texture.clone().unwrap()));
        self
    }

    fn color_texture(mut self, color: Color) -> Self {
        self.texture = Some(ColorTexture::new(color));
        self
    }
}

impl ShapeBuilder<Metal<ColorTexture>, ColorTexture> {
    fn metal(mut self, fuzz: f64) -> Self {
        self.material = Some(Metal::new(self.texture.clone().unwrap(), fuzz));
        self
    }

    fn color_texture(mut self, color: Color) -> Self {
        self.texture = Some(ColorTexture::new(color));
        self
    }
}

impl ShapeBuilder<Dielectric, ColorTexture> {
    fn dielectric(mut self, ri: f64) -> Self {
        self.material = Some(Dielectric::new(ri));
        self
    }
}

struct RandomScene {
    world: ShapeList,
}

impl RandomScene {
    fn new() -> Self {
        let mut world = ShapeList::new();

        world.push(
            ShapeBuilder::<Lambertian<_>, _>::new()
                .color_texture(Color::new(0.8, 0.8, 0.8))
                .lambertian()
                .sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0)
                .build(),
        );

        world.push(
            ShapeBuilder::<Lambertian<_>, _>::new()
                .color_texture(Color::new(1.0, 0.2, 0.2))
                .lambertian()
                .sphere(Point3::new(-6.0, 3.0, 0.0), 3.0)
                .build(),
        );
        world.push(
            ShapeBuilder::new()
                .dielectric(1.5)
                .sphere(Point3::new(0.0, 3.0, 0.0), 3.0)
                .build(),
        );
        world.push(
            ShapeBuilder::<Metal<_>, _>::new()
                .color_texture(Color::new(0.8, 0.8, 0.8))
                .metal(0.4)
                .sphere(Point3::new(6.0, 3.0, 0.0), 3.0)
                .build(),
        );

        Self { world }
    }

    fn background(&self, direction: &Vec3) -> Color {
        let t = 0.5 * (direction.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl SceneWithDepth for RandomScene {
    fn camera(&self) -> Camera {
        Camera::from_lookat(
            Point3::new(0.0, 9.0, 12.0),
            Point3::zero(),
            Vec3::yaxis(),
            50.0,
            self.aspect(),
        )
    }

    fn trace(&self, ray: &Ray, depth: usize) -> Color {
        // floatの誤差を除外するため、0.0に近いtを解としない
        // つまり、tが0.0に近いと衝突したと判定されず、直接self.background()へ行く
        // 0.5掛けされないので暗くならない
        let hit_info = self.world.hit(ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            if depth == 0 {
                return Color::zero();
            }

            if let Some(scatter) = hit.m.scatter(ray, &hit) {
                scatter.albedo * self.trace(&scatter.ray, depth - 1)
            } else {
                Color::zero()
            }
        } else {
            self.background(&ray.direction)
        }
    }

    fn width(&self) -> u32 {
        200
    }

    fn height(&self) -> u32 {
        100
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    render_aa_width_depth("render.png", RandomScene::new())?;
    Ok(())
}
