use std::fs::File;
use std::io::Write;

use rt::camera::*;
use rt::light::*;
use rt::ray::*;
use rt::vec3::*;
use rt::world::*;

use rt::cylindre::*;
use rt::plane::*;
use rt::sphere::*;
use rt::cube::*;
use indicatif::{ProgressBar,ProgressStyle};

// fn ray_color(ray: &Ray, world: &World, lights: &[Light]) -> Vec3 {
//     let background_color = Vec3::new(0.5, 0.7, 1.0);

//     match world.hit(ray, 0.001, f64::INFINITY) {
//         Some(hit_record) => {
//             let mut total_color = Vec3::new(0.0, 0.0, 0.0);

//             for light in lights {
//                 let light_color = calculate_lighting(&hit_record, light);
//                 total_color = total_color.add(&light_color);
//             }

//             total_color
//         }
//         None => {
//             let unit_direction = ray.direction;
//             let t = 0.5 * (unit_direction.y + 1.0);
//             background_color
//                 .mul(1.0 - t)
//                 .add(&Vec3::new(1.0, 1.0, 1.0).mul(t))
//         }
//     }
// }


fn ray_color(ray: &Ray, world: &World, lights: &[Light]) -> Vec3 {
    let background_color = Vec3::new(0.5, 0.7, 1.0);
    
    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            let mut total_color = Vec3::new(0.0, 0.0, 0.0);
            
            for light in lights {
                // Passage du world en paramètre
                let light_color = calculate_lighting(&hit_record, light, world);
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
    // Creation de la barre de progression
    let progress_bar = ProgressBar::new((height*width) as u64);
    progress_bar.set_style(ProgressStyle::default_bar()
    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pixels ({eta})")
    .unwrap()
    .progress_chars("=>-"));
    // Création du World et ajout des objets
    let mut world = World::new();

       // Ajout de plans
       world.add(Box::new(Plane::new(
        Vec3::new(0.0, -0.5, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.5, 0.5, 0.5),
    )));


    // Ajout de plusieurs sphères
    // world.add(Box::new(Sphere::new(
    //     Vec3::new(0.0, 0.0, -1.5),
    //     0.5,
    //     Vec3::new(0.8, 0.3, 0.3),
    // )));

    world.add(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, -1.5),
        0.8,
        Vec3::new(1.0, 1.0, 0.0),
    )));

    
    // Ajout de cylindres
    // world.add(Box::new(Cylinder::new(
    //     Vec3::new(0.0, -0.5, -3.0), // Base
    //     Vec3::new(0.0, 1.0, 0.0),   // Axe parallèle à Y
    //     0.5,                        // Rayon
    //     1.0,                        // Hauteur
    //     Vec3::new(0.3, 0.3, 0.8),   // Couleur
    // )));


    // Ajout d'un cube avec des dimensions plus visibles
    // world.add(Box::new(Cube::new(
    //     Vec3::new(-1.0, -0.5, -2.0),    // Point minimum
    //     Vec3::new(0.0, 0.5, -1.0),      // Point maximum
    //     Vec3::new(0.8, 0.6, 0.2),       // Couleur (doré)
    // )));

    // Création des lumières
    let lights = vec![
        Light::new(Vec3::new(5.0, 5.0, -3.0), 0.8),
        Light::new(Vec3::new(-5.0, 5.0, -3.0), 0.6),
        Light::new(Vec3::new(0.0, 5.0, 0.0), 0.4),
    ];

    let camera = Camera::new(
        Vec3::new(3.0, 2.0, 6.0),     // Position: légèrement en hauteur et en retrait
        Vec3::new(0.0, 0.0, -1.0),    // Point de visée: centre de la scène
        Vec3::new(0.0, 1.0, 0.0),     // Vecteur "up"
        60.0,                         // Angle de champ plus large
        (width as f64) / (height as f64),
    );



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

            // Incrémenter la barre de progression
            progress_bar.inc(1);
        }
    }
    Ok(())
}
