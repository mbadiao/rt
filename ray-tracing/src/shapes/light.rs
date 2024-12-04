use super::vec3::{Point3, Vec3, dot};
use super::color::Color;

pub struct Light {
    pub position: Point3,
    intensity: f64,
    color: Color,
}

impl Light {
    pub fn new(position: Point3, intensity: f64, color: Color) -> Self {
        Light {
            position,
            intensity,
            color,
        }
    }

    pub fn calculate_lighting(&self, point: Point3, normal: Vec3) -> Color {
        let light_dir = (self.position - point).normalize();
        let distance = (self.position - point).length();
        
        // Calcul de l'atténuation basée sur la distance
        let attenuation = self.intensity / (distance * distance);
        
        // Calcul de l'éclairage diffus (loi de Lambert)
        let n_dot_l = dot(normal, light_dir).max(0.0);
        
        self.color * (attenuation * n_dot_l)
    }
}