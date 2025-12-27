# Gestionnaire d'image BMP

Bibliothèque Rust pour créer et lire des images BMP à partir d'une matrice RGBA via FFI C.


***

## ✨ Fonctionnalités

- ✅ **Écriture** : Matrice RGBA → Fichier BMP 24-bit
- ✅ **Lecture** : Fichier BMP → Matrice RGBA (alpha=255)
- ✅ **Tests unitaires** inclus

***

## 📋 API C

### Écriture BMP

```c
int write_bmp_from_rgba_matrix(
    const uint8_t* rgba_matrix,  // RGBA plat (width × height × 4)
    uint32_t width,              // Largeur pixels
    uint32_t height,             // Hauteur pixels
    const char* file_path        // Chemin fichier BMP
);
// Retour: 0=OK, -1=Erreur
```


### Lecture BMP

```c
int read_bmp_to_rgba_matrix(
    const char* file_path,       // Chemin BMP
    uint32_t* width,             // OUT: largeur
    uint32_t* height,            // OUT: hauteur
    uint8_t* rgba_matrix,        // OUT: RGBA plat
    uint64_t max_size            // Taille buffer max
);
// Retour: 0=OK, -1=Erreur, -2=Buffer trop petit
```


***

## 🚀 Exemple d'utilisation C

```c
#include "bmp_image_manager.h"
#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>

int main() {
    // === ÉCRITURE ===
    const uint32_t w = 640, h = 480;
    uint8_t* rgba = malloc(w * h * 4);
    
    // Remplir rouge opaque
    for (uint32_t i = 0; i < w * h * 4; i += 4) {
        rgba[i + 0] = 255;  // R
        rgba[i + 1] = 0;    // G  
        rgba[i + 2] = 0;    // B
        rgba[i + 3] = 255;  // A
    }
    
    int res = write_bmp_from_rgba_matrix(rgba, w, h, "./output.bmp");
    printf("Écriture: %s\n", res == 0 ? "OK" : "ERREUR");
    free(rgba);
    
    // === LECTURE ===
    uint32_t out_w, out_h;
    uint8_t* buffer = malloc(1920 * 1080 * 4);  // HD buffer
    
    res = read_bmp_to_rgba_matrix("./output.bmp", &out_w, &out_h, 
                                  buffer, 1920ULL * 1080 * 4);
    if (res == 0) {
        printf("Lecture OK: %dx%d\n", out_w, out_h);
    } else if (res == -2) {
        printf("Buffer trop petit pour %dx%d\n", out_w, out_h);
    }
    
    free(buffer);
    return 0;
}
```


***

## 🔨 Compilation \& Exécution


### 1. Compiler le programme C

```bash
gcc main.c -Ltarget/release -lbmp_image_manager -o main -lm
```


### 2. Exécuter

```bash
LD_LIBRARY_PATH=target/release ./main
```


***

## 📁 Structure du projet

```
bmp_image_manager/
├── Cargo.toml
├── src/lib.rs          # Code Rust + tests
├── bmp_image_manager.h # Header C généré
├── target/release/
│   ├── libbmp_image_manager.so  # Linux
│   └── libbmp_image_manager.dylib # macOS
└── README.md
```


***



## 📄 Licence MIT

```
Copyright (c) 2025 theora59

Permission is hereby granted, free of charge, to any person obtaining a copy...
```

Merci de me contacter si vous voulez de l'aide sou souhaitez apporter/que j'apporte des modifications à cette lib.

***
