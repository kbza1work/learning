use std::os::raw::c_void;
use std::path::Path;

use gl;

use image;
use image::GenericImage;
use image::DynamicImage::*;

pub fn load_texture(path: &str, flip_y: bool) -> u32 {
    let mut texture_id = 0;

    unsafe {
        gl::GenTextures(1, &mut texture_id);
        let mut img = image::open(&Path::new(path)).expect(&format!("Texture {} failed to load", path));

        if flip_y {
            img = img.flipv(); // flip loaded texture on the y-axis.
        }
        let img = img;

        let format = match img {
            ImageLuma8(_) => gl::RED,
            ImageLumaA8(_) => gl::RG,
            ImageRgb8(_) => gl::RGB,
            ImageRgba8(_) => gl::RGBA,
        };

        let data = img.raw_pixels();

        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexImage2D(gl::TEXTURE_2D, 0, format as i32, img.width() as i32, img.height() as i32,
            0, format, gl::UNSIGNED_BYTE, &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        gl::BindTexture(gl::TEXTURE_2D, 0);

        texture_id
    }
}
