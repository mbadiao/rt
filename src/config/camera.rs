
use super::vec3::*;
use super::ray::*;

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
}