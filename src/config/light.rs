// use super::vec3::*;
// use super::hittable::*;


// pub struct Light {
//     pub position: Vec3,
//     pub intensity: f64,
// }

// impl Light {
//    pub fn new(position: Vec3, intensity: f64) -> Self {
//         Light {
//             position,
//             intensity,
//         }
//     }
// }

// pub fn calculate_lighting(hit_record: &HitRecord, light: &Light) -> Vec3 {
//     let light_direction = light.position.sub(&hit_record.point).normalize();
//     let normal = hit_record.normal;

//     let diffuse_intensity = light.intensity * normal.dot(&light_direction).max(0.0);

//     hit_record.color.mul(diffuse_intensity)
// }

use super::vec3::*;
use super::hittable::*;
use super::ray::Ray;
use super::world::World;

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

pub fn calculate_lighting(hit_record: &HitRecord, light: &Light, world: &World) -> Vec3 {
    // Vérification des ombres
    let shadow_ray_direction = light.position.sub(&hit_record.point);
    let shadow_ray_distance = shadow_ray_direction.length();
    let shadow_ray = Ray {
        origin: hit_record.point.add(&hit_record.normal.mul(0.001)), // Légère offset pour éviter l'auto-intersection
        direction: shadow_ray_direction.normalize(),
    };

    // Si un objet bloque le chemin vers la lumière, le point est dans l'ombre
    if let Some(_shadow_hit) = world.hit(&shadow_ray, 0.001, shadow_ray_distance) {
        return hit_record.color.mul(0.1); // Retourne uniquement la composante ambiante
    }

    // Composante ambiante - lumière de base
    let ambient_strength = 0.1;
    let ambient = hit_record.color.mul(ambient_strength);

    // Composante diffuse - réflexion de la lumière sur la surface
    let light_direction = shadow_ray_direction.normalize();
    let normal = hit_record.normal;
    let diff = normal.dot(&light_direction).max(0.0);
    let diffuse = hit_record.color.mul(diff * light.intensity);

    // Composante spéculaire - reflets brillants
    // let view_direction = hit_record.point.mul(-1.0).normalize();
    // let reflect_direction = reflect(&light_direction.mul(-1.0), &normal);
    
    // let specular_strength = 0.5;
    // let shininess = 32.0;
    // let spec = reflect_direction.dot(&view_direction)
    //     .max(0.0)
    //     .powf(shininess);
    // let specular = Vec3::new(1.0, 1.0, 1.0)
    //     .mul(spec * specular_strength * light.intensity);

    // Combinaison des trois composantes
    ambient.add(&diffuse)
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.sub(&n.mul(2.0 * v.dot(n)))
}