use super::*;

#[test]
fn heightmap_for_square_peak() {
    let (actual_vertices, actual_indices, actual_normals) =
        heightmap_data("test/heightmap/test_heightmap.png", true);

    let expected_vertices: Vec<f32> = vec![
        // row 1
        0.0,  -32_513.0,  0.0,
        0.5,  -32_513.0,  0.0,
        1.0,  -32_513.0,  0.0,
        // row 2
        0.0,  -32_513.0,  0.5,
        0.5,   32_512.0,  0.5,
        1.0,  -32_513.0,  0.5,
        // row 3
        0.0,  -32_513.0,  1.0,
        0.5,  -32_513.0,  1.0,
        1.0,  -32_513.0,  1.0,
    ];
    let expected_normals = vec![
        // row 1
        0.0,        1.0,  0.0,
        0.0,   65_025.0, -4.0,
        0.0,        1.0,  0.0,
        // row 2
        0.0,  -32_513.0, -4.0,
        0.0,        1.0,  0.0,
        0.0,  -32_513.0, -4.0,
        // row 3
        0.0,  -32_513.0, -4.0,
        0.0,  -32_513.0, -4.0,
        0.0,  -32_513.0, -4.0,
    ];
    let expected_indices: Vec<i32> = vec![
        // strip 1
        0, 1, 3,
        1, 4, 3,

        1, 2, 4,
        2, 5, 4,

        // strip 2
        3, 4, 6,
        4, 7, 6,

        4, 5, 7,
        5, 8, 7,
    ];

    assert_eq!(actual_vertices, expected_vertices);
    assert_eq!(actual_indices, expected_indices);
}

#[test]
fn heightmap_for_rectangular_peak() {
    let (actual_vertices, actual_indices, actual_normals) =
        heightmap_data("test/heightmap/test_heightmap_2.png", true);

    let expected_vertices: Vec<f32> = vec![
        // row 1
        0.0,  -7067.609375,     0.0,
        0.2,  -7067.609375,     0.0,
        0.4,  -7067.609375,     0.0,
        0.6,  -7067.609375,     0.0,
        0.8,  -7067.609375,     0.0,
        1.0,  -7067.609375,     0.0,
        // row 2
        0.0,  -7067.609375,     0.5,
        0.2,   18632.78125,     0.5,
        0.4,   32767.99609375,  0.5,
        0.6,   32767.99609375,  0.5,
        0.8,   18632.78125,     0.5,
        1.0,  -7067.609375,     0.5,
        // row 3
        0.0,  -7067.609375,     1.0,
        0.2,   18632.78125,     1.0,
        0.4,   18632.78125,     1.0,
        0.6,   18632.78125,     1.0,
        0.8,   18632.78125,     1.0,
        1.0,  -7067.609375,     1.0,
    ];
    let expected_indices: Vec<i32> = vec![
        // strip 1
        0, 1, 6,
        1, 7, 6,

        1, 2, 7,
        2, 8, 7,

        2, 3, 8,
        3, 9, 8,

        3, 4, 9,
        4, 10, 9,

        4, 5, 10,
        5, 11, 10,

        // strip 2
        6, 7, 12,
        7, 13, 12,

        7, 8, 13,
        8, 14, 13,

        8, 9, 14,
        9, 15, 14,

        9, 10, 15,
        10, 16, 15,

        10, 11, 16,
        11, 17, 16,
    ];

    assert_eq!(actual_vertices, expected_vertices);
    assert_eq!(actual_indices, expected_indices);
}
