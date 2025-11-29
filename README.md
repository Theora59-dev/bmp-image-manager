# Gestionnaire d'image BMP

Bibliothèque Rust pour créer et sauvegarder des images BMP à partir d'une matrice RGBA passée via FFI C.

---

## Présentation

Cette bibliothèque Rust permet de créer une image BMP à partir d'une matrice de pixels au format RGBA reçue depuis un programme en C. Elle expose une fonction compatible C via FFI (Foreign Function Interface) pour faciliter l'interopérabilité.

---

## Fonction principale exposée
```C
int write_bmp_from_rgba_matrix(
const uint8_t* rgba_matrix,
uint32_t width,
uint32_t height,
const char* file_path
);
```

- **rgba_matrix** : Pointeur vers un tableau plat d'octets représentant les pixels RGBA (largeur × hauteur × 4 octets).
- **width** : Largeur de l'image en pixels.
- **height** : Hauteur de l'image en pixels.
- **file_path** : Chemin du fichier BMP où sauvegarder l'image (chaîne C terminée par zéro).
- **retour** : 0 en cas de succès, -1 en cas d'erreur.

---

## Exemple d'utilisation en C
```C
#include "bmp_image_manager.h"
#include <stdint.h>
#include <stdio.h>

int main() {
const uint32_t width = 42;
const uint32_t height = 42;
uint8_t rgba_matrix[width * height * 4];

text
// Remplir la matrice avec du rouge opaque
for (uint32_t i = 0; i < width * height; i++) {
    rgba_matrix[i * 4 + 0] = 255;
    rgba_matrix[i * 4 + 1] = 0; 
    rgba_matrix[i * 4 + 2] = 0; 
    rgba_matrix[i * 4 + 3] = 0; 
}

const char* file_path = "./image.bmp";
int res = write_bmp_from_rgba_matrix(rgba_matrix, width, height, file_path);

if (res == 0) {
    printf("Image bmp créée avec succès.\n");
} else {
    printf("Erreur lors de la création de l'image bmp.\n");
}

return res;

}
```

---

## Compilation et exécution

Compiler le binaire C en le liant avec la bibliothèque Rust :
```bash
gcc votreprogramme.c -Lchemin_vers_le_dossier_contenant -lnom_du_fichier -o nom_de_lexecutable`
```
Exemple: `gcc main.c -Lbmp_image_manager/target/release/ -lbmp_image_manager -o main`


Exécuter en précisant la variable d'environnement pour trouver la librairie dynamique :
- Sous linux: 
```bash
LD_LIBRARY_PATH=chemin_vers_la_librairie ./nom_de_lexecutable
```
- Par exemple:  `LD_LIBRARY_PATH=./bmp_image_manager/target/release ./main`
---

## Fonctionnement interne

- La fonction Rust `write_bmp_from_rgba_matrix` convertit le tableau brut RGBA en image BMP en mémoire.
- Elle utilise la crate Rust [`bmp`](https://crates.io/crates/bmp) pour la gestion des pixels et la sauvegarde du fichier au format BMP.

---

## Licence MIT 
- Merci de me contacter si vous voulez utiliser mon travail.
- Plus d'[informations](https://fr.wikipedia.org/wiki/Licence_MIT)
