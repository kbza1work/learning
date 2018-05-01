#[cfg(test)]
#[path = "./heightmap_tests.rs"]
pub mod heightmap_tests;

use std::path::Path;

use image;
use image::{GenericImage, Pixel};

use cgmath::prelude::*;
use cgmath::Vector3;

type Face = Vec<f32>;
type Normal = Vec<Vector3<f32>>;

/// takes a path to an image file representing a heightmap and a boolean indicating whether the
/// image should be flipped in the y direction, and returns a tuple containing the flattened vertex
/// data, the indices for drawing the vertices, and the flattened normal vectors
pub fn heightmap_data(path: &str, flip_y: bool) -> (Vec<f32>, Vec<i32>, Vec<f32>) {
    let (vertices, heightmap_height, heightmap_width) =
        vertices_from_heightmap(path, flip_y);
    let indices = indices_from_heightmap(heightmap_height, heightmap_width);
    // let normals = normals_from_heightmap(&vertices, heightmap_height, heightmap_width);
    let flattened_vertices: Vec<f32> =
        vertices.into_iter().flat_map(|point| point.into_iter()).collect();
    let flattened_normals = vec![];
    // let flattened_normals: Vec<f32> =
    //     normals.into_iter().flat_map(|normal| normal.into_iter()).collect();

    return (flattened_vertices, indices, flattened_normals);
}

/// returns a tuple of the vertices, the heightmap's height, and the heightmap's width,
/// respectively
fn vertices_from_heightmap(path: &str, flip_y: bool) -> (Vec<Face>, u32, u32) {
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

fn indices_from_heightmap(height: u32, width: u32) -> Vec<i32> {
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

// pub fn normals_from_heightmap(vertices: &Vec<Face>, height: u32, width: u32) -> Vec<Normal> {
//     let mut normals = Vec::with_capacity(vertices.len());

//     for i in 0..vertices.len() {
//         let l, r, t, b = 0;

//         // assume at the edges of the heightmap that the slope is constant
//         if i % width == 0 {
//             l = -(vertices[i + 1][1] - vertices[i][y]);
//         } else {
//             l = vertices[i - 1][1];
//         }
//         if i % width == (width - 1) {
//             r = -(vertices[i - 1][1] - vertices[i][1])
//         } else {
//             r = vertices[i + 1][1];
//         }
//         if i < width {
//             b = -(vertices[i + width][1] - vertices[i][1]);
//         } else {
//             b = vertices[i - width][1];
//         }
//         if i > height * (width - 1) {
//             t = -(vertices[i - width][1] - vertices[i][1]);
//         } else {
//             t = vertices[i + width][1];
//         }

//         normals.push(Vector3::new(2 * (r - l), 2 * (b - t), -4).normalize());
//     }

//     normals
// }
