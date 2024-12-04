use ray_tracing::shapes::camera::Camera;
use ray_tracing::shapes::color;
use ray_tracing::shapes::color::Color;
use ray_tracing::shapes::common;
use ray_tracing::shapes::cylinder::Cylinder;
use ray_tracing::shapes::hittable::{HitRecord, Hittable};
use ray_tracing::shapes::hittable_list::HittableList;
use ray_tracing::shapes::light::Light;
use ray_tracing::shapes::material::{Dielectric, Lambertian, Metal};
use ray_tracing::shapes::ray::Ray;
use ray_tracing::shapes::{cube::Cube, sphere::Sphere};
use ray_tracing::shapes::{vec3, vec3::Point3};
use std::io;
use std::rc::Rc;
// Comprendre les concepts
// Sphère dans un espace 3D : Une sphère est définie par un point central  C et un rayon  R.
//L'équation de la sphère est :
// (P−C)⋅(P−C)=R2 où P est un point sur la surface de la sphère, et ⋅ est le produit scalaire.
// Rayon dans un espace 3D : Un rayon est défini par un point d'origine A et une direction  b.
// Sa position à tout moment est donnée par :
// P(t)=A+t⋅b
// où t (f64) est un scalaire qui détermine à quelle distance le rayon se trouve de son origine.
// Intersection d'un rayon et d'une sphère : Pour déterminer si un rayon touche une sphère, il faut résoudre l'équation suivante :
// (A+t⋅b−C)⋅(A+t⋅b−C)=R2
// Cela revient à résoudre une équation quadratique pour t.

fn ray_color(r: &Ray, world: &dyn Hittable, lights: &[Light], depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let mut rec = HitRecord::new();
    // Élimination de l'acné des ombres
    // Problème : Les rayons réfléchis peuvent heurter la surface d'origine à une distance proche de zéro (
    // t≈0), créant des artefacts visuels appelés acné des ombres.
    // Solution : Ignorer les collisions très proches de t=0.0
    // en utilisant une tolérance minimale (t>0.001).
    if world.hit(r, 0.001, common::INFINITY, &mut rec) {
        // Couleur de base du matériau
        let mut final_color = Color::new(0.0, 0.0, 0.0);

        // Lumière ambiante
        let ambient_strength = 0.1;
        let ambient = rec.mat.as_ref().unwrap().get_color() * ambient_strength;
        final_color += ambient;

        // Contribution de chaque lumière
        for light in lights {
            let to_light = (light.position - rec.p).normalize();
            let shadow_ray = Ray::new(rec.p + rec.normal * 0.001, to_light);
            let mut shadow_rec = HitRecord::new();

            // Vérification des ombres
            if !world.hit(&shadow_ray, 0.001, common::INFINITY, &mut shadow_rec) {
                let light_contribution = light.calculate_lighting(rec.p, rec.normal);
                final_color += light_contribution * rec.mat.as_ref().unwrap().get_color();
            }
        }

        // Réflexions/réfractions existantes
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            final_color += attenuation * ray_color(&scattered, world, lights, depth - 1);
        }

        return final_color;
    }

    // Couleur du fond
    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

// fn random_scene() -> HittableList {
//     let mut world = HittableList::new();

//     let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
//     world.add(Box::new(Sphere::new(
//         Point3::new(0.0, -1000.0, 0.0),
//         1000.0,
//         ground_material,
//     )));

//     for a in -5..5 {
//         for b in -5..5 {
//             let choose_mat = common::random_double();
//             let center = Point3::new(
//                 a as f64 + 0.9 * common::random_double(),
//                 0.2,
//                 b as f64 + 0.9 * common::random_double(),
//             );

//             if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
//                 if choose_mat < 0.8 {
//                     // Diffuse
//                     let albedo = Color::random() * Color::random();
//                     let sphere_material = Rc::new(Lambertian::new(albedo));
//                     world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
//                 } else if choose_mat < 0.95 {
//                     // Metal
//                     let albedo = Color::random_range(0.5, 1.0);
//                     let fuzz = common::random_double_range(0.0, 0.5);
//                     let sphere_material = Rc::new(Metal::new(albedo, fuzz));
//                     world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
//                 } else {
//                     // Glass
//                     let sphere_material = Rc::new(Dielectric::new(1.5));
//                     world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
//                 }
//             }
//         }
//     }

//     let material1 = Rc::new(Dielectric::new(1.5));
//     world.add(Box::new(Sphere::new(
//         Point3::new(0.0, 1.0, 0.0),
//         1.0,
//         material1,
//     )));

//     let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
//     world.add(Box::new(Sphere::new(
//         Point3::new(-4.0, 1.0, 0.0),
//         1.0,
//         material2,
//     )));

//     let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
//     world.add(Box::new(Sphere::new(
//         Point3::new(4.0, 1.0, 0.0),
//         1.0,
//         material3,
//     )));

//     world
// }

fn main() {
    // Image

    // Définit un ratio d’aspect de 16:9 (largeur/hauteur).
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 600;

    //  Dimensions de l'image en pixels. Ici, largeur = 400 pixels, hauteur = 400/(16/9)=225 pixels.
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();

    // Ground material
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));

    // Other materials
    let metal_material = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3)); // gold-like metal
    let _glass_material = Rc::new(Dielectric::new(1.5));
    let _diffus_material = Rc::new(Lambertian::new(Color::new(0.2, 0.3, 0.4)));

    // Large flat cube (ground)
    world.add(Box::new(Cube::new(
        Point3::new(-1000.0, -0.5, -1000.0), // Minimum corner (large negative values)
        Point3::new(1000.0, 0.0, 1000.0),    // Maximum corner (flat ground plane)
        ground_material,
    )));

    // Add geometric shapes on top of the "ground"

    // Sphere 1: Metallic
    // world.add(Box::new(Sphere::new(
    //     Point3::new(0.0, 1.0, -4.0), // Center of the sphere
    //     1.0,                         // Radius
    //     metal_material.clone(),
    // )));

    // Sphere 2: Glass
    // world.add(Box::new(Sphere::new(
    //     Point3::new(-2.0, 1.0, -6.0), // Center of the sphere
    //     1.0,                          // Radius
    //     glass_material.clone(),
    // )));

    // Small Cube: Diffuse
    // world.add(Box::new(Cube::new(
    //     Point3::new(2.0, 0.0, -4.0), // Minimum corner of the small cube
    //     Point3::new(3.0, 1.0, -3.0), // Maximum corner of the small cube
    //     diffus_material.clone(),
    // )));

    // Define the material for the cylinder
    let cylinder_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); // Red diffuse material

    // Define the base center of the cylinder, radius, height, and axis
    let base = Point3::new(4.0, -1.0, -5.0); // The base of the cylinder is at (0, -1, -5)
    let radius = 1.0; // The radius of the cylinder is 1.0
    let height = 3.0; // The height of the cylinder is 3.0
    let axis = vec3::Vec3::new(0.0, 1.0, 0.0); // The cylinder's axis is along the Y-axis

    // Add the cylinder to the world
    world.add(Box::new(Cylinder::new(
        base,
        radius,
        height,
        axis,
        metal_material,
    )));

    // Camera setup
    let lookfrom = Point3::new(10.0, 3.0, 10.0);
    let lookat = Point3::new(0.0, 1.0, -5.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    // Création des lumières
    let lights = vec![
        Light::new(
            Point3::new(10.0, 10.0, 10.0), // Position
            50.0,                          // Intensité
            Color::new(1.0, 1.0, 1.0),     // Couleur blanche
        ),
        Light::new(
            Point3::new(-10.0, 5.0, -10.0),
            30.0,
            Color::new(0.9, 0.8, 0.7), // Couleur chaude
        ),
    ];

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, &lights, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
