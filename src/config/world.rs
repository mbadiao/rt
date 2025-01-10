
use super::hittable::*;
use super::ray::*;

pub struct World {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t = t_max;

        for object in &self.objects {
            if let Some(hit_record) = object.hit(ray, t_min, closest_t) {
                if hit_record.t < closest_t {
                    closest_t = hit_record.t;
                    closest_hit = Some(hit_record);
                }
            }
        }
        closest_hit
    }

}
