# Documentation Ray Tracing - Implémentation du Plan

## Introduction au Plan dans le Ray Tracing

Le plan est l'une des formes géométriques fondamentales dans un système de ray tracing. Il représente une surface infinie définie par un point et une normale, particulièrement utile pour créer des sols, des murs ou d'autres surfaces plates dans une scène 3D.

## Structure du Plan

```rust
pub struct Plane {
    pub point: Vec3,  // Un point appartenant au plan
    pub normal: Vec3, // Vecteur normal au plan (perpendiculaire à la surface)
    pub color: Vec3,  // Couleur du plan
}
```

### Attributs Principaux
- `point`: Un point quelconque appartenant au plan
- `normal`: Le vecteur normal qui définit l'orientation du plan
- `color`: La couleur du plan en format RGB (Vec3)

## Création d'un Plan

### Constructeur
```rust
pub fn new(point: Vec3, normal: Vec3, color: Vec3) -> Self {
    Plane {
        point,
        normal: normal.normalize(), // Normalisation automatique du vecteur normal
        color,
    }
}
```

### Utilisation dans la Scène Principale
```rust
// Exemple d'ajout d'un plan (sol) dans la scène
world.add(Box::new(Plane::new(
    Vec3::new(0.0, -0.5, 0.0),  // Position du plan (légèrement en dessous de l'origine)
    Vec3::new(0.0, 1.0, 0.0),   // Normale orientée vers le haut (plan horizontal)
    Vec3::new(0.5, 0.5, 0.5),   // Couleur grise
)));
```

## Algorithme d'Intersection (hit)

L'algorithme d'intersection est crucial pour le ray tracing. Il détermine si et où un rayon intersecte le plan.

```rust
fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
    let denom = self.normal.dot(&ray.direction);
    
    // Vérification du parallélisme
    if denom.abs() < 1e-6 {
        return None;
    }

    // Calcul du point d'intersection
    let t = self.point.sub(&ray.origin).dot(&self.normal) / denom;
    
    // Vérification des bornes
    if t < t_min || t > t_max {
        return None;
    }

    let point = ray.point_at_parameter(t);
    
    Some(HitRecord {
        t,
        point,
        normal: self.normal,
        color: self.color,
    })
}
```

### Étapes de l'Algorithme d'Intersection

1. **Test de Parallélisme**
   - Calcul du produit scalaire entre la normale du plan et la direction du rayon
   - Si proche de zéro, le rayon est parallèle au plan (pas d'intersection)

2. **Calcul du Point d'Intersection**
   - Utilisation de l'équation du plan et du rayon pour trouver le paramètre t
   - t représente la distance le long du rayon jusqu'au point d'intersection

3. **Validation de l'Intersection**
   - Vérification que t est dans l'intervalle [t_min, t_max]
   - Création d'un HitRecord si l'intersection est valide

