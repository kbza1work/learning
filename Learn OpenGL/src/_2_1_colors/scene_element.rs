use cgmath::Matrix4;

pub trait SceneElement {
    fn render_frame(&self, t: u32, view_matrix: &Matrix4<f32>, projection_matrix: &Matrix4<f32>);
}
