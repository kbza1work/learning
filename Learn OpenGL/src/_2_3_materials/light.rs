use cgmath::{Vector3, Vector4};

pub struct Light {
    pub position: Vector4<f32>,

    pub ambient_color: Vector3<f32>,
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,
}
