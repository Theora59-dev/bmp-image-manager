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
}

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
