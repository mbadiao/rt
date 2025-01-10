# Documentation du Projet

## **Introduction**
Ce projet implémente un système de rendu 3D utilisant le langage Rust. L'objectif principal est de générer des scènes avec différentes formes géométriques, lumières et caméras configurables. Il peut être utilisé comme un point de départ pour des projets de rendu ou comme une étude technique des concepts de base en graphiques 3D.

### **Fonctionnalités principales**
- Gestion de formes 3D (cube, sphère, cylindre, plan).
- Configuration des rayons et des collisions (ray tracing).
- Gestion des lumières et des caméras.
- Scènes personnalisables.

## **Structure du Projet**

### **Aperçu**
```
.
├── Cargo.lock
├── Cargo.toml
├── docs
│   ├── cylindre
│   │   └── doc.md
│   ├── lumiere
│   │   └── doc.md
│   ├── plan
│   │   └── doc.md
│   └── sphere
│       └── doc.md
├── examples
│   ├── all_shape.png
│   ├── Cube.png
│   ├── Cylinder.png
│   ├── Plan.png
│   └── Sphere.png
├── light.svg
├── Makefile
├── readme.md
└── src
    ├── config
    │   ├── camera.rs
    │   ├── hittable.rs
    │   ├── light.rs
    │   ├── mod.rs
    │   ├── ray.rs
    │   ├── vec3.rs
    │   └── world.rs
    ├── lib.rs
    ├── main.rs
    └── shape
        ├── cube.rs
        ├── cylindre.rs
        ├── mod.rs
        ├── plane.rs
        └── sphere.rs
```

# Guide d'Utilisation du Ray Tracer

## Configuration de la Scène de Base

Pour créer une scène de base dans le ray tracer, vous devrez configurer trois éléments essentiels : la résolution de l'image, la caméra, et le monde qui contiendra vos objets.

```rust
// Configuration de base
let width = 800;
let height = 600;
let samples = 10;  // Nombre d'échantillons par pixel pour l'anti-aliasing

// Création du monde
let mut world = World::new();

// Configuration du fichier de sortie
let mut file = File::create("world_scene.ppm")?;
writeln!(file, "P3\n{} {}\n255", width, height)?;
```

## Ajout d'Éléments dans la Scène

### Création d'un Plan (Sol)
```rust
// Création d'un sol gris
world.add(Box::new(Plane::new(
    Vec3::new(0.0, -0.5, 0.0),  // Position (Y négatif pour être sous les objets)
    Vec3::new(0.0, 1.0, 0.0),   // Normale vers le haut
    Vec3::new(0.5, 0.5, 0.5),   // Couleur grise
)));
```


## Création des Formes Géométriques

### 1. Sphères

Les sphères sont définies par leur centre, leur rayon et leur couleur.

```rust
// Création d'une sphère jaune en hauteur
world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 2.0, 0.0),    // Position (centre de la sphère)
    0.2,                          // Rayon
    Vec3::new(1.0, 1.0, 0.0),    // Couleur (jaune)
)));

// Création d'une sphère rouge plus grande
world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.5),   // Position
    0.5,                          // Rayon plus grand
    Vec3::new(0.8, 0.3, 0.3),    // Couleur (rouge)
)));
```

Conseils pour les sphères :
- Variez les tailles pour créer de l'intérêt visuel
- Utilisez différentes hauteurs (coordonnée Y) pour la profondeur
- Évitez les intersections entre sphères pour un rendu plus réaliste

### 2. Cylindres

Les cylindres sont définis par leur base, leur axe, leur rayon, leur hauteur et leur couleur.

```rust
// Création d'un cylindre bleu vertical
world.add(Box::new(Cylinder::new(
    Vec3::new(1.0, -0.5, 2.5),   // Point de base
    Vec3::new(0.0, 1.0, 0.0),    // Axe (vertical)
    0.5,                          // Rayon
    1.0,                          // Hauteur
    Vec3::new(0.3, 0.3, 0.8),    // Couleur (bleu)
)));
```

Conseils pour les cylindres :
- L'axe détermine l'orientation (par défaut vertical avec (0,1,0))
- La base doit être positionnée en tenant compte de la hauteur
- Utile pour créer des piliers, des poteaux ou des tiges

### 3. Cubes

Les cubes sont définis par deux points (min et max) qui forment leur diagonale, ainsi que leur couleur.

```rust
// Création d'un cube doré
let mut cube = Cube::new(
    Vec3::new(-1.0, -0.5, -2.0),  // Point minimum (coin inférieur)
    Vec3::new(0.0, 0.5, -1.0),    // Point maximum (coin supérieur)
    Vec3::new(0.8, 0.6, 0.2),     // Couleur (doré)
);

// Rotation du cube (optionnel)
cube.rotate_y(PI/2.0);  // Rotation de 90 degrés autour de l'axe Y

world.add(Box::new(cube));
```

Conseils pour les cubes :
- Les points min et max définissent la taille et la position
- Utilisez rotate_y pour orienter le cube
- Évitez les dimensions trop extrêmes pour un rendu naturel

## Création de Scènes Complexes

### Exemple de Scène Complète

```rust
// Création du monde
let mut world = World::new();

// Ajout du sol
world.add(Box::new(Plane::new(
    Vec3::new(0.0, -0.5, 0.0),
    Vec3::new(0.0, 1.0, 0.0),
    Vec3::new(0.5, 0.5, 0.5),
)));

// Ajout d'une sphère centrale
world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.5),
    0.5,
    Vec3::new(0.8, 0.3, 0.3),
)));

// Ajout d'un cylindre
world.add(Box::new(Cylinder::new(
    Vec3::new(1.0, -0.5, 2.5),
    Vec3::new(0.0, 1.0, 0.0),
    0.5,
    1.0,
    Vec3::new(0.3, 0.3, 0.8),
)));

// Ajout d'un cube
let mut cube = Cube::new(
    Vec3::new(-1.0, -0.5, -2.0),
    Vec3::new(0.0, 0.5, -1.0),
    Vec3::new(0.8, 0.6, 0.2),
);
world.add(Box::new(cube));

// Configuration de l'éclairage
let lights = vec![
    Light::new(Vec3::new(5.0, 5.0, -5.0), 0.8),
    Light::new(Vec3::new(-5.0, 5.0, -3.0), 0.6),
];
```


### Configuration de l'Éclairage

La luminosité de la scène est contrôlée par les lumières. Vous pouvez ajouter plusieurs sources lumineuses avec différentes intensités.

```rust
let lights = vec![
    Light::new(
        Vec3::new(5.0, 5.0, -5.0), // Position de la lumière
        0.8,                        // Intensité (0.0 à 1.0)
    ),
    // Ajoutez d'autres lumières pour un meilleur éclairage
    Light::new(Vec3::new(-5.0, 5.0, -3.0), 0.6),
];
```

### Configuration de la Caméra

La caméra définit le point de vue de la scène. Vous pouvez la positionner et l'orienter de différentes manières.

```rust
let camera = Camera::new(
    Vec3::new(5.0, 5.0, -5.0),  // Position de la caméra
    Vec3::new(0.0, 0.0, 0.0),   // Point visé (lookAt)
    Vec3::new(0.0, 1.0, 0.0),   // Vecteur "up"
    60.0,                        // Champ de vision (FOV) en degrés
    (width as f64) / (height as f64), // Ratio d'aspect
);
```

#### Positions Recommandées pour la Caméra

1. Vue d'Ensemble
```rust
// Vue globale de la scène
Camera::new(
    Vec3::new(5.0, 5.0, -5.0),  // Position surélevée
    Vec3::new(0.0, 0.0, 0.0),   // Regarde le centre
    Vec3::new(0.0, 1.0, 0.0),   // Up vector
    60.0,                        // FOV large
    aspect_ratio,
)
```

2. Vue Rapprochée
```rust
// Vue plus proche des objets
Camera::new(
    Vec3::new(0.0, 1.0, 5.0),   // Plus proche et plus bas
    Vec3::new(0.0, 0.0, 0.0),   // Regarde le centre
    Vec3::new(0.0, 1.0, 0.0),   // Up vector
    35.0,                        // FOV plus étroit
    aspect_ratio,
)
```

## Contrôle de la Qualité de Rendu

La qualité de l'image finale peut être ajustée via plusieurs paramètres :

```rust
// Anti-aliasing (plus d'échantillons = meilleure qualité mais plus lent)
let samples = 10;  // Augmentez pour une meilleure qualité

// Résolution (plus haute = plus détaillée mais plus lent)
let width = 800;   // Largeur en pixels
let height = 600;  // Hauteur en pixels
```

## Suivi de la Progression

Le ray tracer inclut une barre de progression pour suivre le rendu :

```rust
let progress_bar = ProgressBar::new((height * width) as u64);
progress_bar.set_style(
    ProgressStyle::default_bar()
        .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} pixels ({eta})")
        .unwrap()
        .progress_chars("=>-"),
);
```

## Conseils pour de Meilleurs Rendus

1. **Éclairage**
   - Utilisez plusieurs sources de lumière pour un éclairage plus naturel
   - Ajustez les intensités entre 0.4 et 0.8 pour un bon équilibre

2. **Position de la Caméra**
   - Commencez avec une vue d'ensemble (position élevée)
   - Réduisez le FOV pour un effet plus cinématographique

3. **Qualité vs Performance**
   - Débutez avec des résolutions basses (800x600) pour les tests
   - Augmentez les échantillons progressivement (10-50) selon vos besoins