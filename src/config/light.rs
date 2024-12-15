use super::vec3::*;
use super::hittable::*;


pub struct Light {
    pub position: Vec3,
    pub intensity: f64,
}

impl Light {
   pub fn new(position: Vec3, intensity: f64) -> Self {
        Light {
            position,
            intensity,
        }
    }
}

pub fn calculate_lighting(hit_record: &HitRecord, light: &Light) -> Vec3 {
    let light_direction = light.position.sub(&hit_record.point).normalize();
    let normal = hit_record.normal;

    let diffuse_intensity = light.intensity * normal.dot(&light_direction).max(0.0);

    hit_record.color.mul(diffuse_intensity)
}