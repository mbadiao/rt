# 11 DIELECTRIQUES

# 1. Introduction aux diélectriques
Les diélectriques sont des matériaux transparents comme l'eau, le verre ou le diamant. Quand un rayon de lumière frappe un diélectrique, il se divise en deux parties :

Un rayon réfléchi, qui rebondit sur la surface.
Un rayon réfracté, qui traverse le matériau en changeant de direction.
Pour modéliser cela dans un programme, on choisit aléatoirement entre réflexion et réfraction, et on génère un seul rayon diffusé (réfléchi ou réfracté) à chaque interaction.

## 2. Loi de Snell
La réfraction est décrite par la loi de Snell :
```
η⋅sin(θ)=η′⋅sin(θ′)
```
- η et 𝜂′ sont les indices de réfraction des matériaux (exemple : air = 1.0, verre = 1.3 à 1.7, diamant = 2.4).

- θ et 𝜃′ sont les angles entre les rayons et la normale à la surface.

Quand un rayon entre ou sort d'un matériau, son angle de réfraction 𝜃′ est calculé grâce à :
``` 
sin(θ′) = η′η⋅sin(θ)
```
## 3. Calcul de la direction du rayon réfracté
 Pour trouver la direction du rayon réfracté, on décompose le rayon en deux parties :
 - Une partie perpendiculaire à la normale (𝑅⊥′).
 - Une partie parallèle à la normale (𝑅∥′).

 Les équations sont :
```
R⊥′= η/η'⋅(R+cos(θ)⋅n)
𝑅∥′ = − racine carré(1−∣𝑅⊥′∣2⋅𝑛)
```
- R : direction du rayon incident.
- n : normale à la surface.

Le vecteur réfracté final est la somme des deux composants :
```
R′=R⊥′+R∥′
```
## 4. Fonction pour la réfraction
Dans le code Rust, la fonction suivante calcule la réfraction d’un rayon :

```rust
pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
    let cos_theta = f64::min(dot(-uv, n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.length_squared())) * n;
    r_out_perp + r_out_parallel
}

```

## 5. Réflexion totale interne
Quand un rayon passe d’un matériau dense ``(𝜂>𝜂)`` à un matériau moins dense, il est possible que la réfraction soit impossible. Cela se produit si :
```
sin(θ′)>1
```
Dans ce cas, tout le rayon est réfléchi, un phénomène appelé réflexion totale interne.
## 6. Approximation de Schlick
En réalité, la réflectivité d’un matériau dépend de l’angle d’incidence du rayon. L’approximation de Schlick simplifie ce calcul en utilisant une formule polynomiale rapide :
```
R = R0+(1−R 0)⋅(1−cos(θ))5
```
Où :
```
R0 = (1+η / 1−η)2
```
```rust
fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
}
```

Cette formule est utilisée pour déterminer la probabilité de réflexion vs réfraction.
## 7. Sphère creuse en verre
Une astuce consiste à créer une sphère creuse en verre. Pour cela, on utilise une sphère normale pour la partie externe et une sphère avec un rayon négatif pour la partie interne. Cela inverse les normales pour simuler une cavité interne.
#### EXEMPLE:
```rust
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
```

## 8. Résumé visuel
Les résultats finaux incluent :
- Une sphère en verre qui peut refléter ou réfracter selon les angles.
- Une sphère creuse en verre qui modélise une bulle.

# 12 POSITIONEMENT DE LA CAMERA

# 1. Comprendre le Champ de Vision (Field of View - FOV)
Le champ de vision vertical (vfov) est l'angle que la caméra peut voir verticalement. On l'exprime en degrés, puis on le convertit en radians pour calculer l'ouverture effective. Ce champ de vision est ajustable pour créer des effets de zoom ou de grand-angle :
- Grand-angle : Champ de vision large (exemple : 90°).
- Zoom : Champ de vision étroit (exemple : 20°).

Pour un écran non carré, le champ de vision horizontal est proportionnel à l'aspect ratio de l'image (largeur/hauteur).

#### Calcul du Plan de Vue

La hauteur du plan de vue est donnée par :
```
    h = 2 ⋅ tan(vfov/2)
```
L'aspect ratio permet ensuite de calculer la largeur :
```
    viewport_width=aspect_ratio⋅viewport_height
```
# 2. Construction d'une Caméra de Base
La caméra génère des rayons qui partent de l'origine et traversent le plan de vue. Voici les éléments principaux de la caméra :
- viewport_height et viewport_width : dimensions du plan de vue.
- focal_length : distance entre la caméra et le plan de vue.
- Les rayons générés visent le plan −Z par défaut.

```rust
    impl Camera {
    pub fn new(
        vfov: f64, // Vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Camera {
        let theta = common::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
```

```rust 
    // Camera
 
    let cam = Camera::new(90.0, ASPECT_RATIO);
```
# 3. Ajout de Position et Orientation
Position de la Caméra
Pour positionner la caméra dans un espace 3D, nous définissons deux points :
- ``lookfrom`` : position de la caméra.
- ``lookat`` : point que la caméra regarde.
### Orientation de la Caméra
Pour inclure la rotation de la caméra, nous définissons un vecteur vup (view up), qui indique la direction "vers le haut". Ce vecteur est projeté dans le plan orthogonal à la direction de vue.
Nous utilisons une base orthonormale :
- w = unit_vector(lookfrom−lookat) : direction de vue.
- u = unit_vector(vup×w) : axe horizontal.
- v = w × u : axe vertical.

# 12 Flou de défocalisation

### 1.Qu'est-ce que le Defocus Blur (ou Profondeur de Champ) ?
Dans une caméra réelle, lorsque vous prenez une photo, certains objets dans l'image sont nets tandis que d'autres sont flous. Cela est dû au fait que seuls les objets à une certaine distance de la caméra sont parfaitement focalisés. Les objets trop proches ou trop éloignés apparaissent flous.

Cette profondeur de champ est causée par l'ouverture de l'objectif (l'aperture) et la façon dont les rayons lumineux sont focalisés par la lentille.

Dans un rendu virtuel, nous simulons cet effet pour rendre les scènes plus réalistes.

### 2. Comment est-ce simulé ?
L'effet est simulé en modifiant la manière dont les rayons (qui définissent les couleurs des pixels) sont générés dans la caméra virtuelle.

1. ``Caméra sans blur``
Dans une caméra virtuelle classique (sans profondeur de champ), tous les rayons lumineux partent d’un même point, appelé ``lookfrom``. Chaque rayon traverse un pixel précis de l'image, ce qui donne une image parfaitement nette.

2. ``Caméra avec blur``
Pour simuler le flou, les rayons ne partent plus tous exactement du même point. Ils partent au hasard à l’intérieur d’un disque centré sur le point ``lookfrom``. Plus ce disque est grand (défini par ``aperture``), plus le flou est prononcé.

### 3. Concepts Clés
#### a) Focus Distance et Focus Plane
- Focus Plane : Le plan où tout est parfaitement net. Il se trouve à une certaine distance, appelée focus_dist, par rapport à la caméra.
- Les objets situés sur ce plan sont nets, et les autres apparaissent flous.
#### b) Aperture et Lens Radius
- ``Aperture`` : La taille effective de l'ouverture de la caméra. Plus l’aperture est grande, plus le flou est prononcé.
- ``Lens Radius`` : C'est la moitié de l'aperture, utilisé pour calculer le disque aléatoire à partir duquel les rayons sont émis.

## 4. Comment cela est implémenté dans le code ?
#### a) Génération de points aléatoires dans un disque
La fonction random_in_unit_disk génère des points aléatoires à l'intérieur d'un disque de rayon 1 centré en (0, 0). Cela permet de simuler une ouverture.
```rust
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(
            common::random_double_range(-1.0, 1.0),
            common::random_double_range(-1.0, 1.0),
            0.0,
        );
        if p.length_squared() >= 1.0 {
            continue; // Réessaye si le point est hors du disque
        }
        return p;
    }
}
```
### b) Création de la caméra
Le constructeur de la caméra (``Camera::new``) initialise :
 - Les vecteurs pour l'orientation (``u``, ``v``, ``w``).
 - Le coin inférieur gauche de l'image (``lower_left_corner``).
 - Les dimensions horizontale et verticale.
 - Le rayon de l'objectif (``lens_radius``), qui est dérivé de l’aperture.
 ```rust
    let lens_radius = aperture / 2.0; 
```
### c) Génération d’un rayon
La méthode ``get_ray`` crée un rayon :
 - Elle génère un décalage aléatoire (``offset``) basé sur un point aléatoire dans le disque (``random_in_unit_disk``).
 - Le rayon est ensuite dirigé vers un point sur le plan de l'image.
 ```rust
    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
    let rd = self.lens_radius * vec3::random_in_unit_disk(); // Décalage aléatoire
    let offset = self.u * rd.x() + self.v * rd.y();

    Ray::new(
        self.origin + offset,
        self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
    )
}
 ```

### d) Utilisation dans la scène

Dans le fichier principal (``main.rs``), une caméra avec profondeur de champ est initialisée avec :
 - ``lookfrom`` : Point où se trouve la caméra.
 - ``lookat`` : Point que la caméra regarde.
 - ``vup`` : Vecteur vertical pour orienter la caméra.
 - ``aperture`` : Plus elle est grande, plus le flou est important.
 - ``focus_dist`` : Distance au plan focal.

 ```rust
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Point3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
 
    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0, // Champ de vision vertical
        ASPECT_RATIO, // Ratio largeur/hauteur
        aperture,
        dist_to_focus,
    );
 ```

 ### 5. Effet de l'aperture
 - Si ``aperture = 0``, il n'y a pas de flou (tous les rayons partent d'un point unique).
 - Si ``aperture > 0``, des rayons aléatoires sont générés dans le disque, ce qui crée le flou.


# Implémentation d'un cube dans un moteur de raytracing


## Structure du code

### 1. Classe `Cube`
La classe `Cube` contient :
- `min` : Coin minimum du cube.
- `max` : Coin maximum du cube.
- `mat` : Matériau appliqué au cube (de type `Rc<dyn Material>`).

### 2. Constructeur
La fonction `Cube::new` permet de créer un cube avec les paramètres :
- `min` : Coin minimum.
- `max` : Coin maximum.
- `mat` : Matériau.

### 3. Méthode `hit`
Cette méthode vérifie si un rayon intersecte le cube. Les étapes incluent :
1. Calcul des intersections avec les plans définis par les faces du cube pour chaque axe (x, y, z).
2. Mise à jour des limites `t_min` et `t_max` pour les intersections valides.
3. Rejet si `t_min > t_max` (le rayon a raté le cube).
4. Calcul de la normale sortante et mise à jour des informations dans le `HitRecord`.

---

## Exemple d'utilisation

### Ajouter un cube dans la scène

```rust
let cube_material = Rc::new(Lambertian::new(Color::new(0.2, 0.8, 0.2))); // Matériau vert
let min = Point3::new(-1.0, -1.0, -1.0); // Coin minimum
let max = Point3::new(1.0, 1.0, 1.0); // Coin maximum

world.add(Box::new(Cube::new(
    min,
    max,
    cube_material,
)));


 # Implémentation d'un cylindre dans un moteur de raytracing

---

## Structure du code

### 1. Classe `Cylinder`
La classe `Cylinder` contient les propriétés suivantes :
- `base_center` : Le centre de la base du cylindre.
- `radius` : Le rayon du cylindre.
- `height` : La hauteur du cylindre.
- `axis` : L'axe du cylindre (normalisé).
- `mat` : Le matériau appliqué au cylindre.

### 2. Constructeur
Le constructeur `Cylinder::new` initialise un cylindre avec les paramètres suivants :
- `base` : Le centre de la base.
- `r` : Rayon.
- `h` : Hauteur.
- `axis` : Axe du cylindre (doit être normalisé).
- `m` : Matériau.

### 3. Méthode `hit`
Cette méthode vérifie si un rayon intersecte le cylindre. Les étapes incluent :
1. Calcul de la distance entre l'origine du rayon et la base.
2. Décomposition du rayon en composantes parallèle et perpendiculaire à l'axe du cylindre.
3. Résolution de l'équation quadratique pour vérifier une intersection avec la surface latérale.
4. Vérification que le point d'intersection est dans les limites de la hauteur.
5. Mise à jour des données d'intersection (`HitRecord`).

---

## Exemple d'utilisation

### Ajouter un cylindre dans la scène

```rust
let cylinder_material = Rc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2))); // Matériau rouge
let base = Point3::new(0.0, -1.0, -5.0); // Base du cylindre
let radius = 1.0; // Rayon
let height = 3.0; // Hauteur
let axis = Vec3::new(0.0, 1.0, 0.0); // Axe vertical

world.add(Box::new(Cylinder::new(
    base,
    radius,
    height,
    axis,
    cylinder_material,
)));
