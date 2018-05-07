#[cfg(test)]
#[path = "./heightmap_tests.rs"]
pub mod heightmap_tests;

use std::path::Path;

use image;
use image::{GenericImage, Pixel};
use image::DynamicImage::*;

type Vertex = Vec<f32>;

pub fn heightmap_data(path: &str, flip_y: bool) -> (Vec<f32>, Vec<i32>) {
    let (vertices, heightmap_height, heightmap_width) =
        vertices_from_heightmap(path, flip_y);
    let indices = indices_from_heightmap(&vertices, heightmap_height, heightmap_width);
    let flattened_vertices: Vec<f32> =
        vertices.into_iter().flat_map(|point| point.into_iter()).collect();

    return (flattened_vertices, indices);
}

/// returns a tuple of the vertices, the heightmap's height, and the heightmap's width,
/// respectively
fn vertices_from_heightmap(path: &str, flip_y: bool) -> (Vec<Vertex>, u32, u32) {
    unsafe {
        let mut img = image::open(&Path::new(path)).expect(&format!("Heightmap {} failed to load", path));

        if flip_y {
            img = img.flipv();
        }

        let (width, height) = img.dimensions();
        let vertices = img.to_rgb().enumerate_pixels().map(|(x, y, pixel)| {
            let channels = pixel.channels();
            let mut terrain_height_value = ((channels[0] as f32) * 256.0 + (channels[1] as f32) + (channels[2] as f32)/256.0) - 32_768.0;

            vec![
                (x as f32)/((width - 1) as f32),
                terrain_height_value,
                (y as f32)/((height - 1) as f32),
            ]
        }).collect();

        (vertices, height, width)
    }
}

fn indices_from_heightmap(vertices: &Vec<Vertex>, height: u32, width: u32) -> Vec<i32> {
    let triangle_count = 2 * (width - 1) * (height - 1);
    let mut indices: Vec<i32> = Vec::with_capacity((triangle_count * 3) as usize);
    for i_usize in 0..(width * (height - 1)) {
        let i = i_usize as i32;

        if (i_usize + 1) % width == 0 {
            continue;
        }

        indices.push(i);
        indices.push(i + 1);
        indices.push(i + width as i32);

        indices.push(i + 1);
        indices.push(i + width as i32 + 1);
        indices.push(i + width as i32);
    }

    indices
}
