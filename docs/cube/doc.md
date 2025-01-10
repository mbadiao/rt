# Documentation Ray Tracing - Implémentation du Cube

## Introduction au Cube dans le Ray Tracing

Le cube représente une forme géométrique tridimensionnelle définie par deux points opposés dans l'espace : un point minimum et un point maximum. Cette implémentation inclut des fonctionnalités avancées comme la rotation et la translation, permettant une manipulation dynamique du cube dans la scène.

## Structure du Cube

```rust
pub struct Cube {
    min: Vec3,     // Point minimum (coin inférieur avant gauche)
    max: Vec3,     // Point maximum (coin supérieur arrière droit)
    color: Vec3,   // Couleur du cube
}
```

### Attributs Principaux
- `min`: Point définissant les coordonnées minimales du cube
- `max`: Point définissant les coordonnées maximales du cube
- `color`: Couleur du cube en format RGB (Vec3)

## Création et Manipulation du Cube

### Constructeur
```rust
pub fn new(min: Vec3, max: Vec3, color: Vec3) -> Self {
    Cube { min, max, color }
}
```

### Méthodes de Transformation

#### Translation
```rust
pub fn translate(&mut self, offset: Vec3) {
    self.min = self.min.add(&offset);
    self.max = self.max.add(&offset);
}
```

#### Rotation autour de l'axe Y
```rust
pub fn rotate_y(&mut self, angle: f64) {
    // Calcul des coordonnées de tous les sommets
    let vertices = [
        self.min,
        Vec3::new(self.min.x, self.min.y, self.max.z),
        // [...autres sommets...]
    ];
    
    // Application de la rotation
    let mut rotated_vertices = vec![];
    for v in vertices.iter() {
        let x = v.x * cos_theta + v.z * sin_theta;
        let z = -v.x * sin_theta + v.z * cos_theta;
        rotated_vertices.push(Vec3::new(x, v.y, z));
    }
    
    // Recalcul de la boîte englobante
    // [...calcul des nouveaux min et max...]
}
```

### Utilisation dans la Scène Principale
```rust
// Création d'un cube doré
let mut cube = Cube::new(
    Vec3::new(-1.0, -0.5, -2.0),    // Point minimum
    Vec3::new(0.0, 0.5, -1.0),      // Point maximum
    Vec3::new(0.8, 0.6, 0.2),       // Couleur dorée
);

// Rotation du cube (optionnelle)
cube.rotate_y(PI/2.0);  // Rotation de 90 degrés

world.add(Box::new(cube));
```

## Algorithme d'Intersection (hit)

L'algorithme d'intersection utilise la méthode "slab" qui consiste à tester l'intersection avec les trois paires de plans perpendiculaires formant le cube.

```rust
fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    // Calcul des intersections pour chaque paire de plans
    let mut tmin = (self.min.x - ray.origin.x) / ray.direction.x;
    let mut tmax = (self.max.x - ray.origin.x) / ray.direction.x;
    
    // [...suite du code d'intersection...]
}
```

### Étapes de l'Algorithme d'Intersection

1. **Test des Intervalles**
   - Calcul des points d'intersection pour chaque axe (X, Y, Z)
   - Tri des points d'intersection (proche/loin) pour chaque axe
   - Mise à jour de l'intervalle d'intersection valide

2. **Détermination de la Normale**
   - Identification de la face touchée en premier
   - Attribution de la normale correspondante

3. **Validation de l'Intersection**
   - Vérification que l'intersection est dans les limites t_min/t_max
   - Création du HitRecord avec les informations d'intersection

## Optimisations et Considérations

1. **Performance**
   - L'algorithme "slab" est efficace pour les cubes alignés sur les axes
   - La rotation nécessite un recalcul de la boîte englobante

2. **Précision Numérique**
   - Utilisation d'un seuil (0.0001) pour la détermination des normales
   - Gestion des cas particuliers lors de la rotation

## Intégration dans la Scène

Pour obtenir les meilleurs résultats :

1. **Positionnement**
   - Considérez la taille relative aux autres objets
   - Utilisez la translation pour un placement précis
   - Appliquez la rotation pour une orientation naturelle

2. **Éclairage**
   - Les faces planes créent des ombres nettes
   - Les arêtes mettent en valeur les changements d'éclairage

3. **Transformations**
   - Appliquez les transformations dans l'ordre : rotation puis translation
   - Évitez les rotations multiples pour maintenir la précision

Cette implémentation du cube offre un excellent équilibre entre fonctionnalité et performance, tout en permettant une manipulation flexible dans la scène 3D.