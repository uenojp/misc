use rayt::{Point3, Ray, Vec3};

use crate::material::Material;

pub trait Shape: Sync {
    fn hit(&self, ray: &Ray, t0: f64, t1: f64) -> Option<HitInfo>;
}

pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    pub const fn new(center: Point3, radius: f64, material: M) -> Self {
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
                    0.0,
                    0.0,
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
                    0.0,
                    0.0,
                ));
            }
        }
        None
    }
}

pub struct HitInfo<'a> {
    pub t: f64,
    pub p: Point3,
    pub n: Vec3,
    pub m: &'a dyn Material,
    pub u: f64,
    pub v: f64,
}

impl<'a> HitInfo<'a> {
    pub const fn new(t: f64, p: Point3, n: Vec3, m: &'a dyn Material, u: f64, v: f64) -> Self {
        Self { t, p, n, m, u, v }
    }
}
