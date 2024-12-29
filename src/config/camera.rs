use std::f64::consts::PI;

use super::ray::*;
use super::vec3::*;
#[derive(Debug)]
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let w = lookfrom.sub(&lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom
                .sub(&u.mul(half_width))
                .sub(&v.mul(half_height))
                .sub(&w),
            horizontal: u.mul(2.0 * half_width),
            vertical: v.mul(2.0 * half_height),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner
                .add(&self.horizontal.mul(u))
                .add(&self.vertical.mul(v))
                .sub(&self.origin)
                .normalize(),
        )
    }

    pub fn moves(&mut self) {
        let radius = 5.0; // Rayon de l'orbite de la caméra
        let center = Vec3::new(1.5, 0.0, -1.5); // Centre de la scène

        let angle_degrees = 45.0; // Angle de rotation autour de l'axe Y
        let theta = angle_degrees * PI / 180.0; // Conversion en radians

        // Nouvelle position de la caméra en orbite autour du centre
        // let camera_x = radius * theta.cos();
        // let camera_z = radius * theta.sin();
        // let camera_position = Vec3::new(camera_x, 2.0, camera_z);

        // Calcul de la position de la caméra en orbite verticale
        let camera_y = radius * theta.sin(); // Mouvement vertical
        let camera_z = radius * theta.cos(); // Mouvement en profondeur
        let camera_position = Vec3::new(0.0, camera_y, camera_z);

        self.origin = camera_position;
        self.lower_left_corner = center;
    }
}
