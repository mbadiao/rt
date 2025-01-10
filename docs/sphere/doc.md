# Documentation Ray Tracing - Implémentation de la Sphère

## Introduction à la Sphère dans le Ray Tracing

La sphère représente l'une des formes géométriques les plus fondamentales dans un système de ray tracing. Elle est définie par son centre et son rayon, offrant une forme parfaitement symétrique dans toutes les directions. Sa simplicité mathématique en fait un excellent point de départ pour comprendre les concepts de base du ray tracing.

## Structure de la Sphère

```rust
pub struct Sphere {
    pub center: Vec3,  // Centre de la sphère
    pub radius: f64,   // Rayon de la sphère
    pub color: Vec3,   // Couleur de la sphère
}
```

### Attributs Principaux
- `center`: Position du centre de la sphère dans l'espace 3D
- `radius`: Distance constante du centre à la surface
- `color`: Couleur de la sphère en format RGB (Vec3)

## Création d'une Sphère

### Constructeur
```rust
pub fn new(center: Vec3, radius: f64, color: Vec3) -> Self {
    Sphere {
        center,
        radius,
        color,
    }
}
```

### Utilisation dans la Scène Principale
```rust
// Exemple d'une petite sphère jaune en hauteur
world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 2.0, 0.0),    // Position élevée
    0.2,                          // Petit rayon
    Vec3::new(1.0, 1.0, 0.0),    // Couleur jaune
)));

// Exemple d'une grande sphère rouge
world.add(Box::new(Sphere::new(
    Vec3::new(0.0, 0.0, -1.5),   // Position centrale
    0.5,                          // Grand rayon
    Vec3::new(0.8, 0.3, 0.3),    // Couleur rouge
)));
```

## Algorithme d'Intersection (hit)

L'algorithme d'intersection pour une sphère utilise l'équation quadratique pour déterminer si et où un rayon intersecte la surface de la sphère.

```rust
fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = ray.origin.sub(&self.center);
    let a = ray.direction.dot(&ray.direction);
    let b = oc.dot(&ray.direction);
    let c = oc.dot(&oc) - self.radius * self.radius;
    let discriminant = b * b - a * c;

    if discriminant > 0.0 {
        // Calcul des points d'intersection
        let temp = (-b - discriminant.sqrt()) / a;
        // [Suite du code...]
    }
    None
}
```

### Étapes de l'Algorithme d'Intersection

1. **Calcul des Coefficients**
   - a : Longueur au carré du vecteur direction du rayon
   - b : Produit scalaire entre le vecteur centre-origine et la direction
   - c : Distance au carré entre l'origine du rayon et le centre, moins le rayon au carré

2. **Analyse du Discriminant**
   - Si discriminant > 0 : Deux points d'intersection
   - Si discriminant = 0 : Un point d'intersection (tangent)
   - Si discriminant < 0 : Pas d'intersection

3. **Sélection du Point d'Intersection**
   - Teste d'abord le point le plus proche (-b - sqrt(discriminant))
   - Si hors limites, teste le point le plus éloigné (-b + sqrt(discriminant))

4. **Calcul de la Normale**
   - La normale est le vecteur normalisé allant du centre au point d'intersection

## Optimisations et Considérations

1. **Performance**
   - L'intersection avec une sphère est l'un des calculs les plus efficaces
   - Le calcul de la normale est simple grâce à la symétrie parfaite

2. **Précision Numérique**
   - L'utilisation de t_min évite les auto-intersections
   - La normalisation de la normale assure des calculs d'éclairage précis

## Intégration dans la Scène

Pour obtenir les meilleurs résultats visuels :

1. **Positionnement**
   - Variez les hauteurs pour créer de la profondeur
   - Évitez les intersections entre sphères
   - Considérez la taille relative aux autres objets

2. **Éclairage**
   - Les sphères mettent particulièrement en valeur les effets d'éclairage
   - La normale variant progressivement crée des dégradés naturels
   - Les ombres projetées sont parfaitement circulaires

3. **Composition**
   - Utilisez différentes tailles pour créer une hiérarchie visuelle
   - Combinez avec d'autres formes pour des scènes plus complexes
   - Placez stratégiquement par rapport aux sources de lumière

## Cas d'Utilisation

Les sphères sont particulièrement utiles pour :
- Créer des objets célestes (planètes, étoiles)
- Représenter des objets organiques
- Tester les effets d'éclairage et d'ombrage
- Prototyper rapidement des scènes