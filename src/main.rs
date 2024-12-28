use std::f64::consts::PI;
use std::fs::File;
use std::io::Write;

use rt::camera::*;
use rt::light::*;
use rt::ray::*;
use rt::vec3::*;
use rt::world::*;

use rt::cube::*;
use rt::cylindre::*;
use rt::plane::*;
use rt::sphere::*;

fn ray_color(ray: &Ray, world: &World, lights: &[Light]) -> Vec3 {
    let background_color = Vec3::new(0.5, 0.7, 1.0);

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            let mut total_color = Vec3::new(0.0, 0.0, 0.0);

            for light in lights {
                let light_color = calculate_lighting(&hit_record, light);
                total_color = total_color.add(&light_color);
            }

            total_color
        }
        None => {
            let unit_direction = ray.direction;
            let t = 0.5 * (unit_direction.y + 1.0);
            background_color
                .mul(1.0 - t)
                .add(&Vec3::new(1.0, 1.0, 1.0).mul(t))
        }
    }
}

fn main() -> std::io::Result<()> {
    let width = 800;
    let height = 600;
    let samples = 10;

    let mut file = File::create("world_scene.ppm")?;
    writeln!(file, "P3\n{} {}\n255", width, height)?;

    // Création du World et ajout des objets
    let mut world = World::new();

    // Ajout de plusieurs sphères
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.0),
    //     0.5,
    //     Vec3::new(0.8, 0.3, 0.3),
    // )));
    world.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.5),
        0.3,
        Vec3::new(1.0, 1.0, 0.0),
    )));

    // Ajout de cylindres
    world.add(Box::new(Cylinder::new(
        Vec3::new(0.0, -0.5, -3.0), // Base
        Vec3::new(0.0, 1.0, 0.0),   // Axe parallèle à Y
        0.5,                        // Rayon
        1.0,                        // Hauteur
        Vec3::new(0.3, 0.3, 0.8),   // Couleur
    )));

    // Ajout de plans
    world.add(Box::new(Plane::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.5, 0.5, 0.5),
    )));

    // Ajout d'un cube avec des dimensions plus visibles
    let mut cube = Cube::new(
        Vec3::new(-1.0, -0.5, -2.0), // Point minimum
        Vec3::new(0.0, 0.5, -1.0),   // Point maximum
        Vec3::new(0.8, 0.6, 0.2),    // Couleur (doré)
    );
    // Vecteur de déplacement : déplace le cube de 2 unités vers la droite et 1 unité vers le haut
    let offset = Vec3::new(2.0, 1.0, 0.0);
    cube.translate(offset);

    world.add(Box::new(cube));

    // Création des lumières
    // let lights = vec![
    //     Light::new(Vec3::new(5.0, 5.0, -3.0), 0.8),
    //     Light::new(Vec3::new(-5.0, 5.0, -3.0), 0.6),
    // ];

    // Création des lumières
    // let lights = vec![
    //     Light::new(Vec3::new(-10.0, 0.1, -10.0), 0.8), // Lumière rasante depuis l'horizon
    //     Light::new(Vec3::new(10.0, 0.1, -10.0), 0.6),  // Autre lumière rasante pour équilibre
    // ];

    let lights = vec![
        // Light::new(Vec3::new(5.0, 0.5, -3.0), 0.8), // Lumière depuis la droite
        // Light::new(Vec3::new(-5.0, 0.5, -3.0), 0.6), // Lumière depuis la gauche
        Light::new(Vec3::new(0.0, 2.0, -3.0), 0.8), // Lumière venant d'en bas
        // Light::new(Vec3::new(0.0, 10.0, 0.0), 0.8), // Nouvelle lumière en haut
    ];

    // Définir la caméra avec la position calculée
    // let mut camera = Camera::new(
    //     Vec3::new(0.0, 0.0, 0.0),      // Position orbitale
    //     Vec3::new(1.5, 0.0, -1.5),                // Regarde toujours le centre de la scène
    //     Vec3::new(0.0, 1.0, 0.0), // Orientation "up" de la caméra
    //     45.0,                     // Champ de vision
    //     (width as f64) / (height as f64),
    // );
    // camera.moves();

    // plus haut
    let radius = 5.0; // Rayon de l'orbite de la caméra
    let center = Vec3::new(0.0, 0.0, 0.0); // Centre de la scène

    let angle_degrees = 90.0; // Angle de rotation autour de l'axe Y
    let theta = angle_degrees * PI / 180.0; // Conversion en radians

    // Nouvelle position de la caméra en orbite autour du centre
    let camera_x = radius * theta.cos();
    let camera_z = radius * theta.sin();
    let camera_position = Vec3::new(camera_x, 2.0, camera_z);

    // Définir la caméra avec la position calculée
    let camera = Camera::new(
        camera_position,  // Position orbitale
        center,           // Regarde toujours le centre de la scène
        Vec3::new(0.0, 5.0, 0.0), // Orientation "up" de la caméra
        90.0,             // Champ de vision
        (width as f64) / (height as f64),
    );

    // println!("{:?}",camera);

    for j in (0..height).rev() {
        for i in 0..width {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples {
                let u = (i as f64 + rand::random::<f64>()) / (width as f64);
                let v = (j as f64 + rand::random::<f64>()) / (height as f64);
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color.add(&ray_color(&ray, &world, &lights));
            }

            pixel_color = pixel_color.mul(1.0 / samples as f64);
            let ir = (255.99 * pixel_color.x.sqrt()) as u8;
            let ig = (255.99 * pixel_color.y.sqrt()) as u8;
            let ib = (255.99 * pixel_color.z.sqrt()) as u8;

            writeln!(file, "{} {} {}", ir, ig, ib).unwrap();
        }
    }
    Ok(())
}
