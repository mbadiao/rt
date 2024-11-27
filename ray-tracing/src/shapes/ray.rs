use super::vec3::{Point3, Vec3};

// Un rayon est un concept fondamental dans les systèmes de ray tracing. Mathématiquement, un rayon peut être vu comme une fonction :
// 𝑃 (𝑡) = 𝐴 + 𝑡 ⋅ 𝑏
// P(t) : Position 3D le long d’une ligne dans l’espace.
// A : Origine du rayon.
// b : Direction du rayon (vecteur).
// t : Un paramètre (réel, représenté par f64 dans le code).

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray {
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
