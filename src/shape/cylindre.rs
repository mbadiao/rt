
use crate::config::vec3::*;
use crate::config::hittable::*;
use crate::config::ray::*;

pub struct Cylinder {
    pub base: Vec3,       // Point de base du cylindre
    pub axis: Vec3,       // Axe du cylindre (vecteur direction)
    pub radius: f64,      // Rayon du cylindre
    pub height: f64,      // Hauteur du cylindre
    pub color: Vec3,      // Couleur
}

impl Cylinder {
    pub fn new(base: Vec3, axis: Vec3, radius: f64, height: f64, color: Vec3) -> Self {
        Cylinder {
            base,
            axis: axis.normalize(),
            radius,
            height,
            color,
        }
    }
}


impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin.sub(&self.base);
        let axis_dot_dir = self.axis.dot(&ray.direction);
        let axis_dot_oc = self.axis.dot(&oc);

        let a = ray.direction.sub(&self.axis.mul(axis_dot_dir)).length_squared();
        let b = 2.0 * oc.sub(&self.axis.mul(axis_dot_oc)).dot(&ray.direction.sub(&self.axis.mul(axis_dot_dir)));
        let c = oc.sub(&self.axis.mul(axis_dot_oc)).length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None; // Pas d'intersection
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-b - sqrt_d) / (2.0 * a);

        // Vérifiez si le point est dans les limites du cylindre fini
        let point = ray.point_at_parameter(t);
        let projection_length = self.axis.dot(&point.sub(&self.base));
        if projection_length < 0.0 || projection_length > self.height {
            t = (-b + sqrt_d) / (2.0 * a);
            let point = ray.point_at_parameter(t);
            let projection_length = self.axis.dot(&point.sub(&self.base));
            if projection_length < 0.0 || projection_length > self.height {
                return None;
            }
        }

        if t < t_min || t > t_max {
            return None;
        }

        let point = ray.point_at_parameter(t);
        let projection = self.axis.mul(self.axis.dot(&point.sub(&self.base)));
        let normal = point.sub(&self.base.add(&projection)).normalize();

        Some(HitRecord {
            t,
            point,
            normal,
            color: self.color,
        })
    }
}