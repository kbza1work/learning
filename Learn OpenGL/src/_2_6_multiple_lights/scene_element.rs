use cgmath::Matrix4;

use super::light::Light;

pub trait SceneElement {
    fn render_frame(
        &self,
        t: f32,
        lights: &Vec<Light>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    );
}
