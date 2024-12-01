use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::{dot, unit_vector, Point3, Vec3};
use std::rc::Rc;
use super::material::Material;

pub struct Cylinder {
    base_center: Point3, // Centre de la base du cylindre
    radius: f64,         // Rayon du cylindre
    height: f64,         // Hauteur du cylindre
    axis: Vec3,          // Axe du cylindre
    mat: Rc<dyn Material>, // Matériau du cylindre
}

impl Cylinder {
    // Constructeur pour créer un nouveau cylindre
    pub fn new(base: Point3, r: f64, h: f64, axis: Vec3, m: Rc<dyn Material>) -> Cylinder {
        Cylinder {
            base_center: base,
            radius: r,
            height: h,
            axis: unit_vector(axis), // Normalisation de l'axe du cylindre
            mat: m,
        }
    }
}

impl Hittable for Cylinder {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        // Calcul du vecteur de la base du cylindre au point d'origine du rayon
        let oc = ray.origin() - self.base_center;

        // Décomposition du rayon en composants parallèle et perpendiculaire à l'axe du cylindre
        let ray_dir_parallel = dot(ray.direction(), self.axis) * self.axis; // Composant parallèle
        let ray_dir_perpendicular = ray.direction() - ray_dir_parallel; // Composant perpendiculaire

        let oc_parallel = dot(oc, self.axis) * self.axis; // Composant parallèle de oc
        let oc_perpendicular = oc - oc_parallel; // Composant perpendiculaire de oc

        // Coefficients de l'équation quadratique pour la surface latérale
        let a = ray_dir_perpendicular.length_squared(); // Coefficient de t^2
        let b = 2.0 * dot(ray_dir_perpendicular, oc_perpendicular); // Coefficient de t
        let c = oc_perpendicular.length_squared() - self.radius * self.radius; // Terme constant

        // Résolution de l'équation quadratique (a * t^2 + b * t + c = 0)
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return false; // Pas d'intersection
        }

        let sqrt_d = discriminant.sqrt();

        // Vérification des deux solutions possibles pour t
        let mut t = (-b - sqrt_d) / (2.0 * a);
        if t < t_min || t > t_max {
            t = (-b + sqrt_d) / (2.0 * a);
            if t < t_min || t > t_max {
                return false;
            }
        }

        // Calcul du point d'intersection et vérification des limites en hauteur
        let hit_point = ray.at(t);
        let height_along_axis = dot(hit_point - self.base_center, self.axis);
        if height_along_axis < 0.0 || height_along_axis > self.height {
            return false; // L'intersection est en dehors de la plage de hauteur
        }

        // Mise à jour des informations d'intersection pour la surface latérale
        rec.t = t;
        rec.p = hit_point;
        let outward_normal = (hit_point - self.base_center - height_along_axis * self.axis) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
