# Documentation de la Caméra pour Ray Tracer

## Vue d'ensemble

Cette implémentation définit une caméra pour un ray tracer en Rust, permettant de contrôler le point de vue et la perspective de la scène rendue. La caméra est définie par sa position, son orientation et son champ de vision.

## Structure de la Caméra

```rust
pub struct Camera {
    pub origin: Vec3,            // Position de la caméra
    pub lower_left_corner: Vec3, // Coin inférieur gauche du plan de visualisation
    pub horizontal: Vec3,        // Vecteur horizontal du plan de visualisation
    pub vertical: Vec3,          // Vecteur vertical du plan de visualisation
}
```

## Paramètres Principaux

### lookfrom (Vec3)
- Définit la position de la caméra dans l'espace 3D
- Point depuis lequel la scène est observée
- Exemple : `Vec3::new(0.0, 2.0, 3.0)` place la caméra à la position (0,2,3)

### lookat (Vec3)
- Point vers lequel la caméra est dirigée
- Centre d'intérêt de la vue
- Exemple : `Vec3::new(0.0, 0.0, 0.0)` fait regarder la caméra vers l'origine

### vup (Vec3)
- Vecteur "up" de la caméra
- Définit l'orientation verticale de la caméra
- Généralement (0,1,0) pour une orientation standard
- Influence la rotation de la caméra autour de son axe de visée

### vfov (f64)
- Champ de vision vertical en degrés
- Contrôle l'angle de vue de la caméra
- Valeurs typiques : entre 30° et 90°
- Plus la valeur est grande, plus la vue est "grand angle"

### aspect (f64)
- Ratio largeur/hauteur de l'image
- Exemple : 16.0/9.0 pour une image 16:9
- Influence la forme du plan de visualisation

## Fonctionnalités Principales

### Constructeur (new)
```rust
pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64) -> Self
```
Le constructeur calcule :
1. La base orthonormée (u,v,w) définissant l'orientation de la caméra
2. Les dimensions du plan de visualisation basées sur le champ de vision
3. Les vecteurs définissant le plan de visualisation

### Génération de Rayons
```rust
pub fn get_ray(&self, u: f64, v: f64) -> Ray
```
- Génère un rayon partant de l'origine de la caméra
- Paramètres u,v : coordonnées normalisées (entre 0 et 1) sur le plan de visualisation
- Retourne le rayon correspondant au pixel (u,v) de l'image

### Animation de la Caméra
```rust
pub fn moves(&mut self)
```
- Permet de déplacer la caméra en orbite autour de la scène
- Rotation de 45° autour de l'axe Y
- Rayon d'orbite fixé à 1.0
- Maintient une hauteur fixe de 2.0

## Calculs Mathématiques Clés

1. Calcul du champ de vision :
   ```rust
   let theta = vfov * std::f64::consts::PI / 180.0;
   let half_height = (theta / 2.0).tan();
   let half_width = aspect * half_height;
   ```

2. Construction de la base orthonormée :
   ```rust
   let w = lookfrom.sub(&lookat).normalize();  // Direction opposée à la visée
   let u = vup.cross(&w).normalize();          // Vecteur horizontal
   let v = w.cross(&u);                        // Vecteur vertical final
   ```

## Bonnes Pratiques d'Utilisation

1. Choisir le `lookfrom` et `lookat` pour cadrer correctement la scène
2. Ajuster le `vfov` selon l'effet de perspective désiré
3. Maintenir un `vup` cohérent pour éviter les rotations non désirées
4. Utiliser un aspect ratio correspondant aux dimensions finales de l'image

## Limitations et Considérations

- La caméra est statique une fois créée (sauf utilisation de `moves()`)
- L'animation est limitée à une orbite simple
- Pas de gestion de la profondeur de champ (depth of field)
- La normalisation des rayons peut impacter légèrement les performances