use super::hittable::{HitRecord, Hittable};
use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use std::rc::Rc;
use super::material::Material;

pub struct Cube {
    min: Point3, // Coin minimum du cube
    max: Point3, // Coin maximum du cube
    mat: Rc<dyn Material>, // Matériau du cube
}

impl Cube {
    // Constructeur pour créer un cube
    pub fn new(min: Point3, max: Point3, mat: Rc<dyn Material>) -> Cube {
        Cube { min, max, mat }
    }
}

impl Hittable for Cube {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut t_min = t_min; // Limite inférieure pour les intersections valides
        let mut t_max = t_max; // Limite supérieure pour les intersections valides

        // Parcourir chaque axe (x, y, z)
        for i in 0..3 {
            let inv_d = 1.0 / r.direction()[i]; // Inverse de la direction du rayon pour cet axe
            let t0 = (self.min[i] - r.origin()[i]) * inv_d; // Intersection avec le plan "min" de l'axe
            let t1 = (self.max[i] - r.origin()[i]) * inv_d; // Intersection avec le plan "max" de l'axe

            // Échanger t0 et t1 si nécessaire (si le rayon va dans la direction négative)
            let (t0, t1) = if inv_d < 0.0 { (t1, t0) } else { (t0, t1) };

            // Mettre à jour les limites t_min et t_max
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);

            // Si t_min dépasse t_max, le rayon a raté le cube
            if t_max <= t_min {
                return false;
            }
        }

        // Mise à jour des informations de l'intersection
        rec.t = t_min; // Plus proche point d'intersection valide
        rec.p = r.at(rec.t); // Point d'intersection

        // Calcul de la normale sortante
        let outward_normal = Vec3::new(
            (rec.p.x() - self.min.x()).abs().min((rec.p.x() - self.max.x()).abs()),
            (rec.p.y() - self.min.y()).abs().min((rec.p.y() - self.max.y()).abs()),
            (rec.p.z() - self.min.z()).abs().min((rec.p.z() - self.max.z()).abs()),
        ).normalize();

        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone()); // Copie du matériau
        true
    }
}
