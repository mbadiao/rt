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
│   ├── doc.md
│   └── readme.md
├── examples
│   ├── Cube.png
│   ├── Cylinder.png
│   ├── Plan.png
│   └── Sphere.png
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

### **Description des dossiers et fichiers**

#### **Fichiers racine**
- **`Cargo.toml`** : Définit les dépendances et les informations sur le projet.
- **`Cargo.lock`** : Verrouille les versions exactes des dépendances.

#### **`docs/`**
Contient la documentation utilisateur et technique :
- **`doc.md`** : Documentation technique principale.
- **`readme.md`** : Introduction et instructions de base.

#### **`examples/`**
Contient des exemples visuels du rendu :
- **`Cube.png`, `Cylinder.png`, `Plan.png`, `Sphere.png`** : Illustrations des formes disponibles.

#### **`src/`**
Contient le code source principal du projet :

##### **Dossier `config/`**
Modules gérant les éléments de configuration :
- **`camera.rs`** : Gestion de la caméra (position, orientation, champ de vision).
- **`hittable.rs`** : Gestion des objets pouvant être "touchés" par un rayon.
- **`light.rs`** : Gestion des sources lumineuses.
- **`vec3.rs`** : Manipulation des vecteurs 3D.
- **`ray.rs`** : Définition des rayons dans l'espace 3D.
- **`world.rs`** : Gestion de la scène globale (objets, lumières, caméras).

##### **Dossier `shape/`**
Modules gérant les formes géométriques :
- **`cube.rs`** : Définition d'un cube.
- **`cylindre.rs`** : Définition d'un cylindre.
- **`plane.rs`** : Définition d'un plan.
- **`sphere.rs`** : Définition d'une sphère.
- **`mod.rs`** : Point d'entrée pour accéder à toutes les formes.

##### **Fichiers principaux**
- **`main.rs`** : Point d'entrée du programme.
- **`lib.rs`** : Module principal contenant les fonctions réutilisables.

## **Détails Techniques**

### **1. Module `config`**
- **Caméra (`camera.rs`)** :
  - Définit les paramètres de la caméra (position, orientation, champ de vision).
  - Calcul des rayons à partir des pixels.

- **Rayons (`ray.rs`)** :
  - Gère les rayons dans l'espace 3D.
  - Inclut la logique pour interagir avec les objets.

- **Vecteurs (`vec3.rs`)** :
  - Fournit des opérations mathématiques sur les vecteurs 3D (addition, produit scalaire, etc.).

- **Lumières (`light.rs`)** :
  - Modélisation des sources lumineuses (intensité, position, couleur).

- **Scène (`world.rs`)** :
  - Contient tous les objets et les éléments lumineux.

### **2. Module `shape`**
- Fournit les différents types de formes géométriques :
  - Cube, Sphère, Cylindre, Plan.
  - Chaque fichier définit les propriétés et les méthodes associées à sa forme.

## **Utilisation**

### **Compilation**
1. Assurez-vous que Rust est installé (à l'aide de [rustup](https://rustup.rs/)).
2. Clonez le projet :
   ```bash
   git clone <url-du-repo>
   cd rt
   ```
3. Compilez le projet :
   ```bash
   cargo build --release
   ```

### **Exécution**
1. Lancez le programme :
   ```bash
   cargo run
   ```
2. Visualisez les résultats dans les fichiers de sortie ou utilisez une interface graphique si prévue.

## **Exemples**

### **Rendus disponibles**
- **Cube** : Voir `examples/Cube.png`.
- **Cylindre** : Voir `examples/Cylinder.png`.
- **Plan** : Voir `examples/Plan.png`.
- **Sphère** : Voir `examples/Sphere.png`.

## **Contributions**
1. Forkez le projet.
2. Créez une branche pour vos modifications :
   ```bash
   git checkout -b feature/mon-feature
   ```
3. Envoyez une pull request après avoir testé vos modifications.

## **Annexes**
- Documentation Rust : [https://doc.rust-lang.org](https://doc.rust-lang.org).
- Concepts de rendu 3D : [https://en.wikipedia.org/wiki/3D_rendering](https://en.wikipedia.org/wiki/3D_rendering).
