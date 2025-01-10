## **Introduction**
Documentation des formes dans le projet de ray tracing
Cette documentation explique comment sont modélisées et utilisées les formes géométriques (sphères, plans, cylindres, cubes, etc.) dans le moteur de ray tracing.

## 1. Sphère

## Fichier : sphere.rs

### Rôle

Le fichier modélise une sphère, l'une des formes les plus basiques du moteur.

### Structure : Sphere

### Champs

* center: Vec3 : Centre de la sphère dans l'espace 3D.
* radius: f64 : Rayon de la sphère.
* color: Vec3 : Couleur de la sphère.
### Méthodes

1. new

    Crée une sphère avec un centre, un rayon et une couleur spécifiés.

```rust
pub fn new(center: Vec3, radius: f64, color: Vec3) -> Self {
    Sphere { center, radius, color }
}
```

### Trait : Hittable
La sphère implémente le trait Hittable, permettant de détecter les intersections avec les rayons.

### 1. Méthode hit
Calcule si un rayon intersecte la sphère. Si oui, retourne un HitRecord avec les informations de l'intersection.

### Étapes principales

* Calcul du discriminant pour déterminer si le rayon touche la sphère.
* Si l'intersection existe, calcule le point d'intersection, la normale, et la couleur.
### 2. Plan

### Fichier : plane.rs
### Rôle
Modélise un plan infini dans l'espace 3D.

### Structure : Plane
### Champs

* point: Vec3 : Un point sur le plan.
* normal: Vec3 : Le vecteur normal au plan (direction perpendiculaire).
* color: Vec3 : Couleur du plan.
### Méthodes
1. new   
    Crée un plan avec un point, une normale et une couleur spécifiés.
```rust
pub fn new(point: Vec3, normal: Vec3, color: Vec3) -> Self {
    Plane { point, normal, color }
}
```



# Documentation : Création des éléments dans le moteur de rendu ray tracing

Ce document explique comment les différents éléments (objets, lumières, caméra, etc.) sont créés et ajoutés à la scène dans le moteur de rendu ray tracing.

---

## 1. Initialisation des Paramètres de Rendu

- **Dimensions de l'image** : `width` et `height` définissent la largeur et la hauteur de l'image en pixels.
- **Nombre d'échantillons** : `samples` définit le nombre d'échantillons par pixel pour l'anti-aliasing.

```rust
let width = 800;
let height = 600;
let samples = 10;
```

---

## 2. Création du Monde

Le `World` est une collection d'objets rendables. Voici comment les différents objets sont ajoutés :

### a. Ajout d'un Plan

Un plan est créé avec :

- Une position (`Vec3`)
- Une normale (`Vec3`)
- Une couleur (`Vec3`)

```rust
world.add(Box::new(Plane::new(
    Vec3::new(0.0, -0.5, 0.0), // Position
    Vec3::new(0.0, 1.0, 0.0),  // Normale
    Vec3::new(0.5, 0.5, 0.5),  // Couleur
)));
```

### b. Ajout de Sphères

Les sphères sont définies par :

- Le centre (`Vec3`)
- Le rayon (`f64`)
- La couleur (`Vec3`)

```rust
world.add(Box::new(Sphere::new(
    Vec3::new(2.5, 0.2, -1.0), // Centre
    0.8,                        // Rayon
    Vec3::new(1.0, 1.0, 0.0),  // Couleur
)));
```

### c. Ajout de Cylindres

Les cylindres nécessitent :

- Une base (`Vec3`)
- Une direction (axe) (`Vec3`)
- Un rayon (`f64`)
- Une hauteur (`f64`)
- Une couleur (`Vec3`)

```rust
world.add(Box::new(Cylinder::new(
    Vec3::new(1.0, -0.5, 2.5), // Base
    Vec3::new(0.0, 1.0, 0.0),   // Axe
    0.5,                        // Rayon
    1.0,                        // Hauteur
    Vec3::new(0.3, 0.3, 0.8),   // Couleur
)));
```

### d. Ajout de Cubes

Les cubes sont définis par :

- Un point minimum (`Vec3`)
- Un point maximum (`Vec3`)
- Une couleur (`Vec3`)

```rust
let mut cube = Cube::new(
    Vec3::new(-1.0, -0.5, -2.0), // Point minimum
    Vec3::new(0.0, 0.5, -1.0),   // Point maximum
    Vec3::new(0.8, 0.6, 0.2),    // Couleur
);
world.add(Box::new(cube));
```

---

## 3. Ajout des Lumières

Les sources de lumière sont ajoutées avec :

- Une position (`Vec3`)
- Une intensité (`f64`)

```rust
let lights = vec![
    Light::new(Vec3::new(5.0, 5.0, -3.0), 0.8),
];
```

---

## 4. Configuration de la Caméra

La caméra est créée avec :

- Une position (`Vec3`)
- Un point de visée (`Vec3`)
- Un vecteur "up" (`Vec3`)
- Un champ de vision (FOV) en degrés (`f64`)
- Un ratio d'aspect (`f64`)

```rust
let camera = Camera::new(
    Vec3::new(3.0, 2.0, 6.0),     // Position
    Vec3::new(0.0, 0.0, -1.0),    // Point de visée
    Vec3::new(0.0, 1.0, 0.0),     // Vecteur "up"
    60.0,                         // Champ de vision
    (width as f64) / (height as f64),
);
```

---

## 5. Processus de Rendu

Pour chaque pixel de l'image :

1. On calcule des rayons à l'aide de la caméra.
2. On échantillonne plusieurs couleurs (anti-aliasing).
3. On combine les couleurs pour obtenir la couleur finale.

Exemple du calcul de couleur d'un rayon :

```rust
fn ray_color(ray: &Ray, world: &World, lights: &[Light]) -> Vec3 {
    let background_color = Vec3::new(0.5, 0.7, 1.0);

    match world.hit(ray, 0.001, f64::INFINITY) {
        Some(hit_record) => {
            let mut total_color = Vec3::new(0.0, 0.0, 0.0);
            for light in lights {
                let light_color = calculate_lighting(&hit_record, light, world);
                total_color = total_color.add(&light_color);
            }
            total_color
        }
        None => {
            let unit_direction = ray.direction;
            let t = 0.5 * (unit_direction.y + 1.0);
            background_color.mul(1.0 - t).add(&Vec3::new(1.0, 1.0, 1.0).mul(t))
        }
    }
}
```

---

## 6. Génération de l'Image

L'image est sauvegardée dans un fichier PPM (Portable Pixel Map).

```rust
let mut file = File::create("world_scene.ppm")?;
writeln!(file, "P3\n{} {}\n255", width, height)?;
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
```

---

## Conclusion

Cette documentation explique comment créer et ajouter différents éléments à la scène. Vous pouvez personnaliser la scène en ajoutant des objets, en modifiant les lumières ou en ajustant les paramètres de la caméra pour créer des rendus uniques.

