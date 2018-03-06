use cgmath::{Vector3, Vector4};

pub struct Light {
    pub position: Vector4<f32>,
    pub color: Vector3<f32>,
    pub ambient_fraction: f32,
}
