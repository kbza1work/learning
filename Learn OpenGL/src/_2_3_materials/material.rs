use cgmath::Vector3;

#[derive(Debug)]
pub struct Material {
    pub ambient_color: Vector3<f32>,
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,
    pub shininess: f32,
}
