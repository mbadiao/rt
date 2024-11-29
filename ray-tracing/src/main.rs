use ray_tracing::shapes::camera::Camera;
use ray_tracing::shapes::color;
use ray_tracing::shapes::color::Color;
use ray_tracing::shapes::common;
use ray_tracing::shapes::hittable::{HitRecord, Hittable};
use ray_tracing::shapes::hittable_list::HittableList;
use ray_tracing::shapes::material::{Dielectric, Lambertian, Metal};
use ray_tracing::shapes::ray::Ray;
use ray_tracing::shapes::sphere::Sphere;
use ray_tracing::shapes::vec3;
use ray_tracing::shapes::vec3::Point3;
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

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
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
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();
        if rec
            .mat
            .as_ref()
            .unwrap()
            .scatter(r, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction = vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    // Définit un ratio d’aspect de 16:9 (largeur/hauteur).
    const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const IMAGE_WIDTH: i32 = 400;
    //  Dimensions de l'image en pixels. Ici, largeur = 400 pixels, hauteur = 400/(16/9)=225 pixels.
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));
    
    // Camera

    let cam = Camera::new();

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
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }
            color::write_color(&mut io::stdout(), pixel_color, SAMPLES_PER_PIXEL);
        }
    }

    eprint!("\nDone.\n");
}
