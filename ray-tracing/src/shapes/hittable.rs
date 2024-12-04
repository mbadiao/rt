use super::material::Material;
use super::ray::Ray;
use super::vec3::{self, Point3, Vec3};
use std::rc::Rc;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub mat: Option<Rc<dyn Material>>,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub ambient_light: f64,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::default(),
            normal: Vec3::default(),
            mat: None,
            t: 0.0,
            front_face: false,
            ambient_light: 0.1, // Valeur par défaut pour la lumière ambiante
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = vec3::dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
}
