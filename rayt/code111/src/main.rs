use rayt::{render_aa, Camera, Float3, Point3, Ray, Scene, Vec3};

trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    // 材質を値で持つか、Rc/Arcで持つか
    // 材質はSphere(dyn Shape)の属性なので、Sphereが値でもって、HitInfoがその参照を持つ形でいいと思う
    material: M,
}

impl<M: Material> Sphere<M> {
    const fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Shape for Sphere<M> {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = ray.direction.dot(oc);
        let c = oc.dot(oc) - self.radius.powi(2);
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let t = (-b - discriminant.sqrt()) / a;
            if t0 < t && t < t1 {
                let p = ray.at(t);
                return Some(HitInfo::new(
                    t,
                    p,
                    (p - self.center) / self.radius,
                    &self.material,
                ));
            }
            let t = (-b + discriminant.sqrt()) / a;
            if t0 < t && t < t1 {
                let p = ray.at(t);
                return Some(HitInfo::new(
                    t,
                    p,
                    (p - self.center) / self.radius,
                    &self.material,
                ));
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

struct HitInfo<'a> {
    t: f64,
    p: Point3,
    n: Vec3,
    m: &'a dyn Material,
}

impl<'a> HitInfo<'a> {
    const fn new(t: f64, p: Point3, n: Vec3, m: &'a dyn Material) -> Self {
        Self { t, p, n, m }
    }
}

#[allow(unused)]
struct ScatterInfo {
    // 散乱した光線
    // \     /
    //  \   / ← direction
    // __\_/__
    //    ↑ origin
    ray: Ray,
    albedo: Float3,
}

impl ScatterInfo {
    const fn new(ray: Ray, albedo: Float3) -> Self {
        Self { ray, albedo }
    }
}

trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> ScatterInfo;
}

struct Lambertian {
    albedo: Float3,
}

impl Lambertian {
    fn new(albedo: Float3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, hit: &HitInfo) -> ScatterInfo {
        // 1. 光線が衝突した点(hit.p)から反射先の点(target)へ
        // 物体は2つの球なので、原点から飛び出した光線は最終的に空へ向かう
        // (振動ぽいことをしない)
        //
        // 2. 原点から最初に飛ばした光線が物体に当たるか当たる場合は必ず0.5掛けされる
        // background()が常にColor::one()を返すようにしても真っ白にはならない
        // (0.5 * Color::one()は灰色)
        let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
        ScatterInfo::new(Ray::new(hit.p, target - hit.p), self.albedo)
    }
}

struct SimpleScene {
    world: ShapeList,
}

impl SimpleScene {
    fn new() -> Self {
        let mut world = ShapeList::new();
        world.push(Box::new(Sphere::new(
            Point3::new(0.0, 0.0, -2.0),
            0.5,
            Lambertian::new(Float3::new(0.8, 0.8, 0.1)),
        )));
        world.push(Box::new(Sphere::new(
            Point3::new(1.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Float3::new(1.0, 0.2, 0.2)),
        )));
        world.push(Box::new(Sphere::new(
            Point3::new(-1.0, 0.0, -1.0),
            0.5,
            Lambertian::new(Float3::new(0.1, 0.2, 0.5)),
        )));
        world.push(Box::new(Sphere::new(
            Point3::new(0.0, -100.5, -1.0),
            100.0,
            Lambertian::new(Float3::new(0.8, 0.8, 0.8)),
        )));
        Self { world }
    }

    fn background(&self, direction: &Vec3) -> Float3 {
        let t = 0.5 * (direction.normalize().y() + 1.0);
        Float3::one().lerp(Float3::new(0.5, 0.7, 1.0), t)
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

    fn trace(&self, ray: &Ray) -> Float3 {
        // floatの誤差を除外するため、0.0に近いtを解としない
        // つまり、tが0.0に近いと衝突したと判定されず、直接self.background()へ行く
        // 0.5掛けされないので暗くならない
        let hit_info = self.world.hit(ray, 0.001, f64::MAX);
        if let Some(hit) = hit_info {
            let scatter_info = hit.m.scatter(ray, &hit);
            scatter_info.albedo * self.trace(&scatter_info.ray)
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
