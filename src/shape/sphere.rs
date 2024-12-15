
use crate::config::vec3::*;
use crate::config::hittable::*;
use crate::config::ray::*;
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub color: Vec3,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, color: Vec3) -> Self {
        Sphere {
            center,
            radius,
            color,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.sub(&self.center);
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);
                let normal = point.sub(&self.center).mul(1.0 / self.radius);
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    color: self.color,
                });
            }
            let temp = (-b + (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = ray.point_at_parameter(temp);
                let normal = point.sub(&self.center).mul(1.0 / self.radius);
                return Some(HitRecord {
                    t: temp,
                    point,
                    normal,
                    color: self.color,
                });
            }
        }
        None
    }
}
