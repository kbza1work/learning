use cgmath::{Deg, Vector3, Vector4};

#[derive(Debug, Copy, Clone)]
pub struct Light {
    pub position: Vector4<f32>,
    pub ambient_color: Vector3<f32>,
    pub diffuse_color: Vector3<f32>,
    pub specular_color: Vector3<f32>,

    pub light_type: LightType,
}

#[derive(Debug, Copy, Clone)]
pub enum LightType {
    Point {
        // attenuation
        constant: f32,
        linear: f32,
        quadratic: f32,
    },
    Directional,
    Spotlight {
        direction: Vector4<f32>,
        inner_angle: Deg<f32>,
        outer_angle: Deg<f32>,

        // if true, this light's position and direction are set to match the camera's position and
        // direction
        flashlight: bool,   
    },
}
