use cgmath::Vector4;

#[derive(Debug, PartialEq)]
pub struct Light {
    position: Vector4<f32>,
    color: [f32; 4],
}

impl Light {
    pub fn new(position: Vector4<f32>, color: [f32; 4]) -> Self {
        Light {
            position: position,
            color: color,
        }
    }

    pub fn position(&self) -> &Vector4<f32> {
        &self.position
    }

    pub fn color(&self) -> &[f32; 4] {
        &self.color
    }
}
