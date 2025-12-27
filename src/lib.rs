use bmp::{Image, Pixel};
use std::ffi::CStr;
use std::os::raw::c_char;

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn create_image() {
        let width = 42;
        let height = 42;
        let color = (255, 0, 255, 0);

        let mut rgba_flat: Vec<u8> = Vec::with_capacity((width * height * 4) as usize);

        for _x in 0..height {
            for _y in 0..width {
                rgba_flat.push(color.0);
                rgba_flat.push(color.1);
                rgba_flat.push(color.2);
                rgba_flat.push(color.3);
            }
        }

        let path = CString::new("./image.bmp").unwrap();
        let ret = write_bmp_from_rgba_matrix(
            rgba_flat.as_ptr(),
            width as u32,
            height as u32,
            path.as_ptr(),
        );

        assert_eq!(ret, 0);
    }

    #[test]
    fn read_image() {
        // Créer une image test
        let test_path = CString::new("./test_image.bmp").unwrap();
        let width = 10u32;
        let height = 10u32;
        let color = (255, 128, 0, 255);

        // Écrire une image test
        let mut rgba_test: Vec<u8> = vec![0; (width * height * 4) as usize];
        for i in (0..rgba_test.len()).step_by(4) {
            rgba_test[i] = color.0;
            rgba_test[i + 1] = color.1;
            rgba_test[i + 2] = color.2;
            rgba_test[i + 3] = color.3;
        }

        write_bmp_from_rgba_matrix(rgba_test.as_ptr(), width, height, test_path.as_ptr());

        // Lire l'image
        let mut out_width = 0u32;
        let mut out_height = 0u32;
        let mut rgba_out: Vec<u8> = vec![0; 10000]; // Buffer assez grand

        let ret = read_bmp_to_rgba_matrix(
            test_path.as_ptr(),
            &mut out_width,
            &mut out_height,
            rgba_out.as_mut_ptr(),
            10000,
        );

        assert_eq!(ret, 0);
        assert_eq!(out_width, width);
        assert_eq!(out_height, height);

        // Vérifier les premiers pixels
        assert_eq!(rgba_out[0], 255);
        assert_eq!(rgba_out[1], 128);
        assert_eq!(rgba_out[2], 0);
        assert_eq!(rgba_out[3], 255);
    }
}

/// Crée un fichier BMP 24-bit à partir d'une matrice RGBA plate (row-major).
///
/// # Arguments
/// * `rgba_matrix` - Pointeur vers la matrice RGBA (R,G,B,A x width x height)
/// * `width` - Largeur de l'image en pixels
/// * `height` - Hauteur de l'image en pixels
/// * `file_path` - Chemin du fichier de sortie (C-string nul-terminé)
///
/// # Retour
/// * `0` en cas de succès
/// * `-1` en cas d'erreur (pointeur nul, chemin invalide, écriture échouée)
///
/// # Exemple
/// ```
/// uint8_t rgba[100*100*4]; // Matrice 100x100 RGBA
/// // Remplir rgba...
/// int result = write_bmp_from_rgba_matrix(rgba, 100, 100, "./output.bmp");
/// ```
#[unsafe(no_mangle)]
pub extern "C" fn write_bmp_from_rgba_matrix(
    rgba_matrix: *const u8,
    width: u32,
    height: u32,
    file_path: *const c_char,
) -> i32 {
    if rgba_matrix.is_null() || file_path.is_null() {
        return -1;
    }

    let slice_size = (width * height * 4) as usize;
    let pixels = unsafe { std::slice::from_raw_parts(rgba_matrix, slice_size) };

    let c_str = unsafe { CStr::from_ptr(file_path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    let mut img = Image::new(width, height);

    for x in 0..height as usize {
        for y in 0..width as usize {
            let i = (x * width as usize + y) * 4; // Position de l'information
            let r = pixels[i];
            let g = pixels[i + 1];
            let b = pixels[i + 2];
            let pixel = Pixel::new(r, g, b);
            img.set_pixel(y as u32, x as u32, pixel);
        }
    }

    match img.save(path_str) {
        Ok(_) => 0,
        Err(_) => -1,
    }
}

/// Charge un fichier BMP et remplit une matrice RGBA plate (row-major).
///
/// # Arguments
/// * `file_path` - Chemin du fichier BMP à charger (C-string)
/// * `width` - Pointeur vers u32 pour recevoir la largeur
/// * `height` - Pointeur vers u32 pour recevoir la hauteur
/// * `rgba_matrix` - Pointeur vers buffer de sortie RGBA
/// * `max_size` - Taille max du buffer en octets
///
/// # Retour
/// * `0` - Succès
/// * `-1` - Erreur (fichier introuvable, pointeur nul, chemin invalide)
/// * `-2` - Buffer trop petit (width/height mis à jour)
///
/// # Notes
/// - Supporte BMP 24-bit (alpha fixé à 255)
#[unsafe(no_mangle)]
pub extern "C" fn read_bmp_to_rgba_matrix(
    file_path: *const c_char,
    width: *mut u32,
    height: *mut u32,
    rgba_matrix: *mut u8,
    max_size: usize,
) -> i32 {
    if file_path.is_null() || width.is_null() || height.is_null() || rgba_matrix.is_null() {
        return -1;
    }

    let c_str = unsafe { CStr::from_ptr(file_path) };
    let path_str = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => return -1,
    };

    // Charger l'image BMP avec bmp::open()
    let img = match bmp::open(path_str) {
        Ok(img) => img,
        Err(_) => return -1,
    };

    let img_width = img.get_width();
    let img_height = img.get_height();
    let required_size = (img_width * img_height * 4) as usize;

    if required_size > max_size {
        unsafe {
            *width = img_width;
            *height = img_height;
        }
        return -2;
    }

    unsafe {
        *width = img_width;
        *height = img_height;
    }

    let mut index = 0;
    for x in 0..img_height as usize {
        for y in 0..img_width as usize {
            let pixel = img.get_pixel(y as u32, x as u32);
            unsafe {
                *rgba_matrix.offset(index as isize) = pixel.r;
                *rgba_matrix.offset((index + 1) as isize) = pixel.g;
                *rgba_matrix.offset((index + 2) as isize) = pixel.b;
                *rgba_matrix.offset((index + 3) as isize) = 255;
            }
            index += 4;
        }
    }

    0
}
