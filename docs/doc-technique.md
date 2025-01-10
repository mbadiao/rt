# Documentation Technique du Projet

## Introduction

Cette documentation technique explique en détail les implémentations de chaque fichier du projet. Elle permet de comprendre la logique et les fonctionnalités développées pour chaque composant, en partant du fichier principal main.rs jusqu'aux différents modules et sous-modules.

## Fichier : main.rs

### Rôle

Le fichier main.rs constitue le point d'entrée principal du programme. Il coordonne l'exécution globale et assemble les différents modules pour générer une scène 3D.

### Fonctionnalités principales

* Initialise les configurations de la scène (caméra, lumières, objets).

* Lance le processus de rendu via le moteur de ray tracing.

* Gère les sorties (affichage ou enregistrement).

### Structure du code

1. Importation des modules principaux (à partir de src/lib.rs ou directement depuis src).

2. Définition des paramètres de la scène.

3. Boucle principale qui calcule les interactions rayon-objet.

### Points d’attention

* La gestion des erreurs doit être adaptée pour chaque étape critique (initialisation, rendu, exportation).

* Possibilité d’ajouter des options via des arguments de ligne de commande.

## Fichier : lib.rs

### Rôle

Il sert de point d'entrée modulaire pour réexporter les fonctionnalités communes à tout le projet.

### Fonctionnalités principales

* Aggrège les modules (config, formes, scène, etc.).

* Fournit des fonctions utilitaires communes.

### Structure du code

* Inclut les modules principaux via des macros mod.

* Définit des alias ou des réexportations pour simplifier les appels (e.g., pub use).

## Module : config

### 1. camera.rs

### Rôle

Modélise la caméra pour observer la scène. Elle gère la position, l’orientation et la projection des rayons dans l’espace 3D.

### Fonctionnalités principales

* Génération des rayons pour chaque pixel.

* Support des champs de vision personnalisés.

### Méthodes clés

* new: Initialise une caméra avec des paramètres donnés.

* get_ray: Calcule un rayon à partir des coordonnées de l’écran.

### 2. ray.rs

### Rôle

Représente un rayon dans l’espace 3D. Les rayons sont les éléments centraux du moteur de ray tracing.

### Fonctionnalités principales

* Définition de l’origine et de la direction.

* Calcul des points le long d’un rayon (à un paramètre donné).

### Méthodes clés

* point_at_parameter: Retourne un point le long du rayon pour un t donné.

### 3. vec3.rs

### Rôle

Fournit une structure pour manipuler les vecteurs 3D (position, direction, couleur).

### Fonctionnalités principales

* Opérations vectorielles : addition, produit scalaire, produit vectoriel, normalisation.

* Présentation des vecteurs sous forme RGB pour les couleurs.

### Méthodes clés

* dot: Produit scalaire.

* cross: Produit vectoriel.

* unit_vector: Normalise un vecteur.

### 4. light.rs

### Rôle

Représente les sources lumineuses de la scène.

### Fonctionnalités principales

* Modélisation des intensités lumineuses.

* Gestion des différents types de lumières (ponctuelles, directionnelles, ambiantes).

## Module : shape

### 1. sphere.rs

### Rôle

* Implémente une sphère en 3D et sa logique d’interaction avec les rayons.

### Fonctionnalités principales

* Détection des intersections rayon-sphère.

* Calcul des normales pour les effets lumineux.

### Méthodes clés

* hit: Retourne les points d’intersection (s’il y en a).

### 2. plane.rs

### Rôle

* Implémente un plan infini en 3D.

### Fonctionnalités principales

* Calcul des intersections rayon-plan.

### Méthodes clés

* hit: Détecte si un rayon coupe le plan.

# Documentation
# Conclusion

Cette documentation technique détaille les principaux composants et leur rôle dans le projet. Pour chaque module, des améliorations potentielles peuvent être envisagées, comme l'ajout de tests unitaires ou de nouvelles primitives 3D.