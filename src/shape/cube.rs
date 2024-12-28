use crate::config::hittable::*;
use crate::config::ray::*;
use crate::config::vec3::*;

pub struct Cube {
    min: Vec3,
    max: Vec3,
    color: Vec3,
}

impl Cube {
    pub fn new(min: Vec3, max: Vec3, color: Vec3) -> Self {
        Cube { min, max, color }
    }

    pub fn translate(&mut self, offset: Vec3) {
        self.min = self.min.add(&offset);
        self.max = self.max.add(&offset);
    }

    pub fn rotate_y(&mut self, angle: f64) {
        let cos_theta = angle.cos();
        let sin_theta = angle.sin();

        // Appliquer la rotation aux sommets du cube
        let vertices = [
            self.min,
            Vec3::new(self.min.x, self.min.y, self.max.z),
            Vec3::new(self.min.x, self.max.y, self.min.z),
            Vec3::new(self.min.x, self.max.y, self.max.z),
            Vec3::new(self.max.x, self.min.y, self.min.z),
            Vec3::new(self.max.x, self.min.y, self.max.z),
            Vec3::new(self.max.x, self.max.y, self.min.z),
            self.max,
        ];

        let mut rotated_vertices = vec![];
        for v in vertices.iter() {
            let x = v.x * cos_theta + v.z * sin_theta;
            let z = -v.x * sin_theta + v.z * cos_theta;
            rotated_vertices.push(Vec3::new(x, v.y, z));
        }

        // Recalculer les nouveaux min et max
        let min_x = rotated_vertices.iter().map(|v| v.x).fold(f64::INFINITY, f64::min);
        let min_y = rotated_vertices.iter().map(|v| v.y).fold(f64::INFINITY, f64::min);
        let min_z = rotated_vertices.iter().map(|v| v.z).fold(f64::INFINITY, f64::min);

        let max_x = rotated_vertices.iter().map(|v| v.x).fold(f64::NEG_INFINITY, f64::max);
        let max_y = rotated_vertices.iter().map(|v| v.y).fold(f64::NEG_INFINITY, f64::max);
        let max_z = rotated_vertices.iter().map(|v| v.z).fold(f64::NEG_INFINITY, f64::max);

        self.min = Vec3::new(min_x, min_y, min_z);
        self.max = Vec3::new(max_x, max_y, max_z);
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
