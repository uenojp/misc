use rayt::{render_aa, Camera, Color, Point3, Ray, Scene, Vec3};

#[derive(Debug)]
struct HitInfo {
    t: f64,
    p: Point3,
    n: Vec3,
}

impl HitInfo {
    const fn new(t: f64, p: Point3, n: Vec3) -> Self {
        Self { t, p, n }
    }
}

trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    const fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Shape for Sphere {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            // Note:
            // t0 <= t && t <= t1 のとき、hitの呼び出し方によっては無限ループになる
            // 例えば、hit(ray, t0 = 0.0, t1 = f64::MAX)をはじめとして、t1を徐々に小さくして狭めていくとき、0.0は固定なので常に解になる。
            if t0 < t && t < t1 {
                let p = ray.at(t);
                return Some(HitInfo::new(t, p, (p - self.center) / self.radius));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t0 < t && t < t1 {
                let p = ray.at(t);
                return Some(HitInfo::new(t, p, (p - self.center) / self.radius));
            }
        }
        None
    }
}

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

struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
        world.push(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
        Self { world }
    }

    fn background(&self, direction: &Vec3) -> Color {
        let t = 0.5 * (direction.normalize().y() + 1.0);
        Color::one().lerp(Color::new(0.5, 0.7, 1.0), t)
    }
}

impl Scene for SimpleScene {
    fn camera(&self) -> rayt::Camera {
        Camera::new(
            Vec3::new(4.0, 0.0, 0.0),
            Vec3::new(0.0, 2.0, 0.0),
            Vec3::new(-2.0, -1.0, -1.0),
        )
    }

    fn trace(&self, ray: &Ray) -> Color {
        let hit_info = self.world.hit(ray, 0.0, f64::MAX);
        if let Some(hit) = hit_info {
            let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
            0.5 * self.trace(&Ray::new(hit.p, target - hit.p))
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
    render_aa("render.png", SimpleScene::new())?;
    Ok(())
}
