use super::vec3::*;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
   pub  fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray {
            origin,
            direction: direction.normalize(),
        }
    }

    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin.add(&self.direction.mul(t))
    }
}