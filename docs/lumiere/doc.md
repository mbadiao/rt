# Documentation Technique du Système d'Éclairage
## Vue d'ensemble du système d'éclairage

Le système d'éclairage implémente le modèle de Phong modifié, offrant un rendu réaliste des surfaces 3D. Il combine trois composantes principales pour simuler différentes interactions lumière-matière.

## Structure de base

### Classe Light
```rust
struct Light {
    position: Vec3,    // Position de la source lumineuse
    intensity: f64     // Intensité (0.0 à 1.0)
}
```

## Composantes de l'éclairage

### 1. Éclairage Ambiant
- **Objectif**: Simuler la lumière indirecte dans l'environnement
- **Implémentation**:
```rust
let ambient_strength = 0.1;
let ambient = hit_record.color.mul(ambient_strength);
```
- **Caractéristiques**:
  - Niveau de base constant (10%)
  - Illumination minimale des zones sombres
  - Indépendant de la position de la source

### 2. Éclairage Diffus
- **Objectif**: Simuler la réflexion de la lumière sur des surfaces mates
- **Implémentation**:
```rust
let light_direction = shadow_ray_direction.normalize();
let diff = normal.dot(&light_direction).max(0.0);
let diffuse = hit_record.color.mul(diff * light.intensity);
```
- **Caractéristiques**:
  - Suit la loi de Lambert (cosinus)
  - Intensité basée sur l'angle d'incidence
  - Dépend de la normale à la surface

### 3. Réflexions Spéculaires
- **Objectif**: Créer des reflets brillants sur les surfaces
- **Implémentation**:
```rust
let view_direction = hit_record.point.mul(-1.0).normalize();
let reflect_direction = reflect(&light_direction.mul(-1.0), &normal);
let spec = reflect_direction.dot(&view_direction)
    .max(0.0)
    .powf(shininess);
```
- **Paramètres clés**:
  - `specular_strength`: Intensité des reflets (0.5 = 50%)
  - `shininess`: Concentration du reflet (32.0 par défaut)

## Gestion des Ombres

### Système de Ray Casting
```rust
let shadow_ray = Ray {
    origin: hit_record.point.add(&hit_record.normal.mul(0.001)),
    direction: shadow_ray_direction.normalize()
};
```

### Détection des obstacles
```rust
if let Some(_shadow_hit) = world.hit(&shadow_ray, 0.001, shadow_ray_distance) {
    return hit_record.color.mul(0.1);
}
```

### Caractéristiques importantes:
- Offset de 0.001 pour éviter l'auto-intersection
- Retour à l'éclairage ambiant en cas d'ombre
- Test des obstacles jusqu'à la source lumineuse

## Calculs Mathématiques Clés

### 1. Normalisation des vecteurs
- **Objectif**: Garantir des calculs cohérents
- **Usage**: 
```rust
direction: shadow_ray_direction.normalize()
```

### 2. Produit Scalaire (dot)
- **Objectif**: Mesurer les angles entre vecteurs
- **Usage**:
```rust
normal.dot(&light_direction).max(0.0)
```

### 3. Réflexion (reflect)
- **Objectif**: Calculer la direction de réflexion
- **Formule**: R = V - 2(V·N)N
  - V: Direction incidente
  - N: Normale à la surface
  - R: Direction réfléchie

## Intégration dans la Scène

### Configuration des Lumières
```rust
let lights = vec![
    Light::new(Vec3::new(5.0, 5.0, -3.0), 0.8),
    Light::new(Vec3::new(-3.0, 3.0, -2.0), 0.6)
];
```

### Calcul de l'Éclairage Final
1. Parcours de toutes les sources lumineuses
2. Accumulation des contributions
3. Combinaison des trois composantes

## Optimisations Possibles

### 1. Performance
- Cache des calculs de normalisation
- Pré-calcul des directions lumineuses
- Parallélisation des calculs

### 2. Qualité
- Support des lumières colorées
- Ajout d'atténuation avec la distance
- Implémentation de soft shadows

## Limitations Actuelles
- Pas de support pour les lumières directionnelles
- Ombres dures uniquement
- Pas de réflexions multiples

## Annexes

### Formules Mathématiques Clés
1. Éclairage diffus: Id = kd(N·L)
2. Éclairage spéculaire: Is = ks(R·V)^n
3. Éclairage total: I = Ia + Id + Is

### Glossaire
- **Normal**: Vecteur perpendiculaire à la surface
- **Shininess**: Exposant spéculaire
- **Ray Casting**: Technique de lancer de rayons
- **Dot Product**: Produit scalaire entre vecteurs