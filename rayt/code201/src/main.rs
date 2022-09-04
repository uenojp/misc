#![allow(dead_code, unused)]

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

// NG
// the trait bound `MaterialType: material::Material` is not satisfied
// the trait `material::Material` is not implemented for `MaterialType`
//
// impl<ShapeType, MaterialType, TextureType> ShapeBuilder<ShapeType, MaterialType, TextureType> {
//     fn sphere(
//         mut self,
//         center: Point3,
//         radius: f64,
//     ) -> ShapeBuilder<Sphere<MaterialType>, MaterialType, TextureType> {
//         ShapeBuilder {
//             shape: Sphere::new(center, radius, self.material),
//             material: self.material,
//             texture: self.texture,
//         }
//     }
// }

// Builder pattern
// ref. https://keens.github.io/blog/2017/02/09/rustnochottoyarisuginabuilderpata_n/
struct ShapeBuilder<ShapeType, MaterialType, TextureType> {
    shape: ShapeType,
    material: MaterialType,
    texture: TextureType,
}

impl ShapeBuilder<(), (), ()> {
    fn new() -> Self {
        Self {
            shape: (),
            material: (),
            texture: (),
        }
    }
}

impl<S: Shape + 'static, M: Material, T: Texture> ShapeBuilder<S, M, T> {
    fn build(self) -> Box<dyn Shape> {
        Box::new(self.shape)
    }
}

impl<ShapeType, MaterialType, TextureType> ShapeBuilder<ShapeType, MaterialType, TextureType> {
    fn color_texture(
        self,
        color: Color,
    ) -> ShapeBuilder<ShapeType, MaterialType, ColorTexture> {
        ShapeBuilder {
            shape: self.shape,
            material: self.material,
            texture: ColorTexture::new(color),
        }
    }
}

impl<ShapeType, MaterialType, T: Texture + Clone> ShapeBuilder<ShapeType, MaterialType, T> {
    fn lambertian(self) -> ShapeBuilder<ShapeType, Lambertian<T>, T> {
        ShapeBuilder {
            shape: self.shape,
            material: Lambertian::new(self.texture.clone()),
            texture: self.texture,
        }
    }

    fn metal(self, fuzz: f64) -> ShapeBuilder<ShapeType, Metal<T>, T> {
        ShapeBuilder {
            shape: self.shape,
            material: Metal::new(self.texture.clone(), fuzz),
            texture: self.texture,
        }
    }

    // fn dielectric(self, ri: f64) -> ShapeBuilder<ShapeType, Dielectric, T> {
    //     ShapeBuilder {
    //         shape: self.shape,
    //         material: Dielectric::new(ri),
    //         texture: self.texture,
    //     }
    // }
}

impl<ShapeType, M: Material + Clone, T: Texture + Clone> ShapeBuilder<ShapeType, M, T> {
    fn sphere(self, center: Point3, radius: f64) -> ShapeBuilder<Sphere<M>, M, T> {
        ShapeBuilder {
            shape: Sphere::new(center, radius, self.material.clone()),
            material: self.material,
            texture: self.texture,
        }
    }
}

// Texture -> Material -> Shape の順を強制
//
// Ok
// let o = ShapeBuilder::new()
//     .color_texture(Color::new(1.0, 0.2, 0.2))
//     .metal(2.1)
//     .sphere(Point3::zero(), 1.0)
//     .build();
//
// Error
// Texture -> Material -> Shape の順でなければいけないが、TextureとMaterialの順が逆
// let o = ShapeBuilder::new()
//     .lambertian();
//     .color_texture(Color::new(1.0, 0.2, 0.2))
//
// Error
// Texture -> Material -> Shape の順でなければいけないが、Shapeがない
// let o = ShapeBuilder::new()
//     .color_texture(Color::new(1.0, 0.2, 0.2))
//     .dielectric(2.1);
//     .build();


struct RandomScene {
    world: ShapeList,
}

impl RandomScene {
    fn new() -> Self {
        let mut world = ShapeList::new();

        world.push(
            ShapeBuilder::new()
                .color_texture(Color::new(0.8, 0.8, 0.8))
                .lambertian()
                .sphere(Point3::new(0.0, -1000.0, 0.0), 1000.0)
                .build(),
        );

        world.push(
            ShapeBuilder::new()
                .color_texture(Color::new(1.0, 0.2, 0.2))
                .lambertian()
                .sphere(Point3::new(-6.0, 3.0, 0.0), 3.0)
                .build(),
        );
        // world.push(
        //     ShapeBuilder::new()
        //         .dielectric(1.5)
        //         .sphere(Point3::new(0.0, 3.0, 0.0), 3.0)
        //         .build(),
        // );
        world.push(
            ShapeBuilder::new()
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
