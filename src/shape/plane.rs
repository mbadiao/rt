
use crate::config::vec3::*;
use crate::config::hittable::*;
use crate::config::ray::*;

pub struct Plane {
    pub point: Vec3,
    pub normal: Vec3,
    pub color: Vec3,
}

impl Plane {
    pub fn new(point: Vec3, normal: Vec3, color: Vec3) -> Self {
        Plane {
            point,
            normal: normal.normalize(),
            color,
        }
    }
}

impl Hittable for Plane {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let denom = self.normal.dot(&ray.direction);
        
        if denom.abs() < 1e-6 {
            return None;
        }

        let t = self.point.sub(&ray.origin).dot(&self.normal) / denom;
        
        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.point_at_parameter(t);
        
        Some(HitRecord {
            t,
            point,
            normal: self.normal,
            color: self.color,
        })
    }
}