
use super::vec3::*;
use super::ray::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f64,
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

// Trait Hittable pour les objets rendables
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
