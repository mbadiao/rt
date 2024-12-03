use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::{dot, unit_vector, Point3, Vec3};
use std::rc::Rc;
use super::material::Material;

pub struct Cylinder {
    pub base_center: Point3,
    pub radius: f64,
    pub height: f64,
    pub axis: Vec3,
    pub mat: Rc<dyn Material>,
}

impl Cylinder {
    pub fn new(base: Point3, r: f64, h: f64, axis: Vec3, m: Rc<dyn Material>) -> Cylinder {
        Cylinder {
            base_center: base,
            radius: r,
            height: h,
            axis: unit_vector(axis),
            mat: m,
        }
    }

    // Helper method to check cap intersections
    fn check_cap_intersection(&self, ray: &Ray, t_min: f64, t_max: f64, is_top: bool) -> Option<f64> {
        let cap_center = if is_top {
            self.base_center + self.axis * self.height
        } else {
            self.base_center
        };

        let t = dot(cap_center - ray.origin(), self.axis) / dot(ray.direction(), self.axis);
        
        if t < t_min || t > t_max {
            return None;
        }

        let hit_point = ray.at(t);
        let distance_from_center = (hit_point - cap_center).length();

        if distance_from_center <= self.radius {
            Some(t)
        } else {
            None
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.base_center;

        // Lateral surface intersection
        let ray_dir_parallel = dot(ray.direction(), self.axis) * self.axis;
        let ray_dir_perp = ray.direction() - ray_dir_parallel;
        
        let oc_parallel = dot(oc, self.axis) * self.axis;
        let oc_perp = oc - oc_parallel;

        let a = ray_dir_perp.length_squared();
        let b = 2.0 * dot(ray_dir_perp, oc_perp);
        let c = oc_perp.length_squared() - self.radius * self.radius;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            // Check caps if no lateral intersection
            if let Some(cap_t) = self.check_cap_intersection(ray, t_min, t_max, false)
                .or_else(|| self.check_cap_intersection(ray, t_min, t_max, true)) {
                let hit_point = ray.at(cap_t);
                let outward_normal = if dot(hit_point - self.base_center, self.axis) < 0.0 {
                    -self.axis
                } else {
                    self.axis
                };

                rec.t = cap_t;
                rec.p = hit_point;
                rec.set_face_normal(ray, outward_normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
            return false;
        }

        let sqrt_d = discriminant.sqrt();
        let mut t = (-b - sqrt_d) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + sqrt_d) / (2.0 * a);
            if t < t_min || t > t_max {
                return false;
            }
        }

        let hit_point = ray.at(t);
        let height_along_axis = dot(hit_point - self.base_center, self.axis);
        
        // Check height limits
        if height_along_axis < 0.0 || height_along_axis > self.height {
            // Check if caps intersect
            if let Some(cap_t) = self.check_cap_intersection(ray, t_min, t_max, height_along_axis > self.height) {
                let cap_hit_point = ray.at(cap_t);
                let outward_normal = if height_along_axis > self.height {
                    self.axis
                } else {
                    -self.axis
                };

                rec.t = cap_t;
                rec.p = cap_hit_point;
                rec.set_face_normal(ray, outward_normal);
                rec.mat = Some(self.mat.clone());
                return true;
            }
            return false;
        }

        // Update hit record for lateral surface
        rec.t = t;
        rec.p = hit_point;
        let outward_normal = (hit_point - self.base_center - height_along_axis * self.axis) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}