use rayt::{Float3, Ray, Vec3};

use crate::shape::HitInfo;
use crate::texture::{ColorTexture, Texture};

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo>;
}

#[derive(Debug, Clone)]
pub struct Lambertian<T: Texture> {
    albedo: T,
}

impl<T: Texture> Lambertian<T> {
    pub fn new(albedo: T) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian<ColorTexture> {
    fn scatter(&self, _: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        // 1. 光線が衝突した点(hit.p)から反射先の点(target)へ
        // 物体は2つの球なので、原点から飛び出した光線は最終的に空へ向かう
        // (振動ぽいことをしない)
        //
        // 2. 原点から最初に飛ばした光線が物体に当たるか当たる場合は必ず0.5掛けされる
        // background()が常にColor::one()を返すようにしても真っ白にはならない
        // (0.5 * Color::one()は灰色)
        let target = hit.p + hit.n + Vec3::random_in_unit_sphere();
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        Some(ScatterInfo::new(Ray::new(hit.p, target - hit.p), albedo))
    }
}

#[derive(Debug, Clone)]
pub struct Metal<T: Texture> {
    albedo: T,
    fuzz: f64,
}

impl<T: Texture> Metal<T> {
    pub const fn new(albedo: T, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal<ColorTexture> {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        let reflected = ray.direction.normalize().reflect(hit.n);
        let reflected = reflected + self.fuzz * Vec3::random_in_unit_sphere();
        let albedo = self.albedo.value(hit.u, hit.v, hit.p);
        if reflected.dot(hit.n) > 0.0 {
            Some(ScatterInfo::new(Ray::new(hit.p, reflected), albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dielectric {
    ri: f64,
}

impl Dielectric {
    pub const fn new(ri: f64) -> Self {
        Self { ri }
    }

    fn schlick(cosine: f64, ri: f64) -> f64 {
        let r0 = ((1.0 - ri) / (1.0 + ri)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitInfo) -> Option<ScatterInfo> {
        // cosineは入射角の余弦
        let (outward_normal, ri, cosine) = {
            let dot = ray.direction.dot(hit.n);
            // 下のcosineを求める式、未理解
            if dot > 0.0 {
                (-hit.n, self.ri, self.ri * dot / ray.direction.length())
            } else {
                (hit.n, self.ri.recip(), -dot / ray.direction.length())
            }
        };
        // note:
        // (-ray.direction)のマイナスはrefractの都合
        // 媒質の境目と光線が通る点を原点として、refractを定義し
        // 光線が媒質1から媒質2へ屈折して進む際、v.reflect(r, ri)のベクトルvとrを向かい合う向きに取っているため
        // ref. テキストのp108
        if let Some(refracted) = (-ray.direction).refract(outward_normal, ri) {
            if Vec3::random_full().x() > Self::schlick(cosine, self.ri) {
                return Some(ScatterInfo::new(Ray::new(hit.p, refracted), Float3::one()));
            }
        }
        let reflected = ray.direction.normalize().reflect(hit.n);
        Some(ScatterInfo::new(Ray::new(hit.p, reflected), Float3::one()))
    }
}

pub struct ScatterInfo {
    // 散乱した光線
    // \     /
    //  \   / ← direction
    // __\_/__
    //    ↑ origin
    pub ray: Ray,
    pub albedo: Float3,
}

impl ScatterInfo {
    pub const fn new(ray: Ray, albedo: Float3) -> Self {
        Self { ray, albedo }
    }
}
