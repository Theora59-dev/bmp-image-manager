#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Crée un fichier BMP 24-bit à partir d'une matrice RGBA plate (row-major).
 *
 * # Arguments
 * * `rgba_matrix` - Pointeur vers la matrice RGBA (R,G,B,A x width x height)
 * * `width` - Largeur de l'image en pixels
 * * `height` - Hauteur de l'image en pixels
 * * `file_path` - Chemin du fichier de sortie (C-string nul-terminé)
 *
 * # Retour
 * * `0` en cas de succès
 * * `-1` en cas d'erreur (pointeur nul, chemin invalide, écriture échouée)
 *
 * # Exemple
 * ```
 * uint8_t rgba[100*100*4]; // Matrice 100x100 RGBA
 * // Remplir rgba...
 * int result = write_bmp_from_rgba_matrix(rgba, 100, 100, "./output.bmp");
 * ```
 */
int32_t write_bmp_from_rgba_matrix(const uint8_t *rgba_matrix,
                                   uint32_t width,
                                   uint32_t height,
                                   const char *file_path);

/**
 * Charge un fichier BMP et remplit une matrice RGBA plate (row-major).
 *
 * # Arguments
 * * `file_path` - Chemin du fichier BMP à charger (C-string)
 * * `width` - Pointeur vers u32 pour recevoir la largeur
 * * `height` - Pointeur vers u32 pour recevoir la hauteur
 * * `rgba_matrix` - Pointeur vers buffer de sortie RGBA
 * * `max_size` - Taille max du buffer en octets
 *
 * # Retour
 * * `0` - Succès
 * * `-1` - Erreur (fichier introuvable, pointeur nul, chemin invalide)
 * * `-2` - Buffer trop petit (width/height mis à jour)
 *
 * # Notes
 * - Supporte BMP 24-bit (alpha fixé à 255)
 */
int32_t read_bmp_to_rgba_matrix(const char *file_path,
                                uint32_t *width,
                                uint32_t *height,
                                uint8_t *rgba_matrix,
                                uintptr_t max_size);
