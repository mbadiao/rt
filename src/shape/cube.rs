use crate::config::vec3::*;
use crate::config::hittable::*;
use crate::config::ray::*;

pub struct Cube {
    min: Vec3,
    max: Vec3,
    color: Vec3,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, color: Vec3) -> Self {
        Cube { min, max, color }
    }
}

impl Hittable for Cube {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
        let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min.y - ray.origin.y) / ray.direction.y;
        let mut tymax = (self.max.y - ray.origin.y) / ray.direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if tmin > tymax || tymin > tmax {
            return None;
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min.z - ray.origin.z) / ray.direction.z;
        let mut tzmax = (self.max.z - ray.origin.z) / ray.direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if tmin > tzmax || tzmin > tmax {
            return None;
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        if tmin < t_max && tmax > t_min {
            let t = tmin;
            let point = ray.point_at_parameter(t);
            
            // Calcul de la normale
            let normal = if (point.x - self.min.x).abs() < 0.0001 {
                Vec3::new(-1.0, 0.0, 0.0)
            } else if (point.x - self.max.x).abs() < 0.0001 {
                Vec3::new(1.0, 0.0, 0.0)
            } else if (point.y - self.min.y).abs() < 0.0001 {
                Vec3::new(0.0, -1.0, 0.0)
            } else if (point.y - self.max.y).abs() < 0.0001 {
                Vec3::new(0.0, 1.0, 0.0)
            } else if (point.z - self.min.z).abs() < 0.0001 {
                Vec3::new(0.0, 0.0, -1.0)
            } else {
                Vec3::new(0.0, 0.0, 1.0)
            };

            return Some(HitRecord {
                t,
                point,
                normal,
                color: self.color,
            });
        }

        None
    }
}


