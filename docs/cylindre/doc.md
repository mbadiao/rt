# Documentation Ray Tracing - Implémentation du Cylindre

## Introduction au Cylindre dans le Ray Tracing

Le cylindre est une forme géométrique tridimensionnelle définie par une base, un axe de direction, un rayon et une hauteur. Il est particulièrement utile pour créer des éléments comme des piliers, des poteaux, ou des tiges dans une scène 3D.

## Structure du Cylindre

```rust
pub struct Cylinder {
    pub base: Vec3,    // Point de base du cylindre
    pub axis: Vec3,    // Axe du cylindre (vecteur direction)
    pub radius: f64,   // Rayon du cylindre
    pub height: f64,   // Hauteur du cylindre
    pub color: Vec3,   // Couleur du cylindre
}
```

### Attributs Principaux
- `base`: Point d'origine du cylindre dans l'espace 3D
- `axis`: Vecteur normalisé définissant la direction et l'orientation du cylindre
- `radius`: Distance du centre à la surface du cylindre
- `height`: Longueur totale du cylindre le long de son axe
- `color`: Couleur du cylindre en format RGB (Vec3)

## Création d'un Cylindre

### Constructeur
```rust
pub fn new(base: Vec3, axis: Vec3, radius: f64, height: f64, color: Vec3) -> Self {
    Cylinder {
        base,
        axis: axis.normalize(), // Normalisation automatique de l'axe
        radius,
        height,
        color,
    }
}
```

### Utilisation dans la Scène Principale
```rust
world.add(Box::new(Cylinder::new(
    Vec3::new(1.0, -0.5, 2.5),  // Base : légèrement enfoncée dans le sol
    Vec3::new(0.0, 1.0, 0.0),   // Axe : vertical
    0.5,                        // Rayon
    1.0,                        // Hauteur
    Vec3::new(0.3, 0.3, 0.8),   // Couleur bleue
)));
```

## Algorithme d'Intersection (hit)

L'algorithme d'intersection pour un cylindre est plus complexe que celui du plan, car il doit gérer à la fois la surface courbe et les limites de hauteur.

```rust
fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let oc = ray.origin.sub(&self.base);
    let axis_dot_dir = self.axis.dot(&ray.direction);
    let axis_dot_oc = self.axis.dot(&oc);

    // Calcul des coefficients de l'équation quadratique
    let a = ray.direction.sub(&self.axis.mul(axis_dot_dir)).length_squared();
    let b = 2.0 * oc.sub(&self.axis.mul(axis_dot_oc))
        .dot(&ray.direction.sub(&self.axis.mul(axis_dot_dir)));
    let c = oc.sub(&self.axis.mul(axis_dot_oc)).length_squared() 
        - self.radius * self.radius;

    // [Suite du code d'intersection...]
}
```

### Étapes de l'Algorithme d'Intersection

1. **Préparation des Calculs**
   - Calcul du vecteur entre l'origine du rayon et la base du cylindre
   - Projection du rayon sur l'axe du cylindre

2. **Résolution de l'Équation Quadratique**
   - Calcul du discriminant pour déterminer l'existence d'intersections
   - Recherche des points d'intersection potentiels

3. **Vérification des Limites**
   - S'assure que les points d'intersection sont dans la hauteur du cylindre
   - Vérifie que l'intersection est dans les limites t_min et t_max

4. **Calcul de la Normale**
   - Détermine le vecteur normal au point d'intersection
   - Crée le HitRecord avec les informations d'intersection

## Conseils d'Utilisation

1. **Positionnement**
   - Base : Placez légèrement en-dessous du sol pour une meilleure intégration
   - Axe : Utilisez (0,1,0) pour un cylindre vertical
   - Ajustez la hauteur et le rayon selon vos besoins

2. **Performance**
   - Le calcul d'intersection est plus coûteux que pour un plan
   - Évitez d'utiliser trop de cylindres dans une même scène

3. **Limitations**
   - Le cylindre est infini en rotation autour de son axe
   - La hauteur est limitée par le paramètre height

## Optimisations et Cas Particuliers

1. **Gestion des Bords**
   - L'algorithme vérifie les intersections avec la surface courbe
   - Les bases circulaires ne sont pas prises en compte dans cette implémentation

2. **Précision Numérique**
   - L'utilisation de normalize() sur l'axe assure la stabilité des calculs
   - Le test du discriminant évite les racines complexes

## Intégration dans la Scène

Pour un meilleur rendu dans le code principal (main.rs) :

1. **Placement Stratégique**
   - Positionnez le cylindre en fonction de sa hauteur
   - Considérez l'interaction avec les autres objets

2. **Éclairage**
   - Les cylindres créent des ombres intéressantes
   - La surface courbe met en valeur les reflets