use cgmath::Vector3;

#[derive(Debug)]
pub struct Material {
    pub ambient_color: Vector3<f32>,
    pub diffuse_map_texture_id: u32,
    pub specular_map_texture_id: u32,
    pub emission_map_texture_id: Option<u32>,
    pub shininess: f32,
}
