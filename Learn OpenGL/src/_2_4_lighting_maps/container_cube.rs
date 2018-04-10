extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::prelude::*;
use cgmath::{Matrix3, Matrix4, Vector3};

use common::shader::Shader;
use common::texture::load_texture;

use super::scene_element::SceneElement;
use super::light::Light;
use super::material::Material;

pub struct ContainerCube {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    model_matrices: Vec<Matrix4<f32>>,
    material: Material,
}

impl ContainerCube {
    pub fn new(src_folder: &str, cube_positions: Vec<Vector3<f32>>) -> Self {

        let (shader_program, vao, vbo) =
            ContainerCube::init_opengl(src_folder);

        let model_matrices =
            cube_positions
            .iter()
            .map(|position| Matrix4::from_translation(*position))
            .collect();

        // changing the order the textures are loaded changes the program output... and crossing
        // the texture IDs produces more correct results than using the right texture IDs... figure
        // out how to better relate sampler2D variables in the fragment shader to shader inputs
        let material = Material {
            ambient_color: Vector3::new(1.0, 0.5, 0.31),
            // DEBUG
            diffuse_map_texture_id: load_texture(&format!("{}/texture_container.png", src_folder), true),
            specular_map_texture_id: load_texture(&format!("{}/texture_container_specular_map.png", src_folder), true),
            emission_map_texture_id: Some(load_texture(&format!("{}/texture_emission_map.jpg", src_folder), false)),
            shininess: 32.0,
        };

        ContainerCube {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            model_matrices: model_matrices,
            material: material,
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);

            let vertices: [f32; 288] = [
                // positions       // surface normals // texture coords
                -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,
                 0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 0.0,
                 0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
                 0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  1.0, 1.0,
                -0.5,  0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0,  0.0, -1.0,  0.0, 0.0,

                -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,
                 0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 0.0,
                 0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
                 0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   1.0, 1.0,
                -0.5,  0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0,  0.0, 1.0,   0.0, 0.0,

                -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,
                -0.5,  0.5, -0.5, -1.0,  0.0,  0.0,  1.0, 1.0,
                -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
                -0.5, -0.5, -0.5, -1.0,  0.0,  0.0,  0.0, 1.0,
                -0.5, -0.5,  0.5, -1.0,  0.0,  0.0,  0.0, 0.0,
                -0.5,  0.5,  0.5, -1.0,  0.0,  0.0,  1.0, 0.0,

                 0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,
                 0.5,  0.5, -0.5,  1.0,  0.0,  0.0,  1.0, 1.0,
                 0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
                 0.5, -0.5, -0.5,  1.0,  0.0,  0.0,  0.0, 1.0,
                 0.5, -0.5,  0.5,  1.0,  0.0,  0.0,  0.0, 0.0,
                 0.5,  0.5,  0.5,  1.0,  0.0,  0.0,  1.0, 0.0,

                -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,
                 0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  1.0, 1.0,
                 0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
                 0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  1.0, 0.0,
                -0.5, -0.5,  0.5,  0.0, -1.0,  0.0,  0.0, 0.0,
                -0.5, -0.5, -0.5,  0.0, -1.0,  0.0,  0.0, 1.0,

                -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0,
                 0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  1.0, 1.0,
                 0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
                 0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  1.0, 0.0,
                -0.5,  0.5,  0.5,  0.0,  1.0,  0.0,  0.0, 0.0,
                -0.5,  0.5, -0.5,  0.0,  1.0,  0.0,  0.0, 1.0
            ];
            let (mut vao, mut vbo) = (0, 0);
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);

            let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

            // positions
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // surface normals
            let surface_normals_offset = (3 * mem::size_of::<GLfloat>()) as *const c_void;
            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, surface_normals_offset);
            gl::EnableVertexAttribArray(1);

            // texture coordinates
            let texture_coords_offset = (6 * mem::size_of::<GLfloat>()) as *const c_void;
            gl::VertexAttribPointer(2, 3, gl::FLOAT, gl::FALSE, stride, texture_coords_offset);
            gl::EnableVertexAttribArray(2);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            (shader_program, vao, vbo)
        }
    }
}

impl SceneElement for ContainerCube {

    fn render_frame(
        &self,
        _t: f32,
        light: &Light,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    ) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            // transforms
            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            // material properties
            // TODO: accept these in the constructor, set them on the struct, and pass to the
            // shader in init_opengl()
            self.shader_program.set_3fv(c_str!("material.ambientColor"), &self.material.ambient_color);
            self.shader_program.set_float(c_str!("material.shininess"), self.material.shininess);

            // light properties
            self.shader_program.set_3fv(c_str!("light.ambientColor"), &light.ambient_color);
            self.shader_program.set_3fv(c_str!("light.diffuseColor"), &light.diffuse_color);
            self.shader_program.set_3fv(c_str!("light.specularColor"), &light.specular_color);
            let light_position_view_space = view_matrix * light.position;
            self.shader_program.set_3fv(c_str!("lightPositionView"), &light_position_view_space.truncate());

            gl::BindVertexArray(self.vao);

            for model_matrix in self.model_matrices.iter() {
                self.shader_program.set_mat4fv(c_str!("modelMatrix"), &model_matrix);

                let normalized_modelview_matrix =
                    (view_matrix * model_matrix)
                    .invert()
                    .expect("Couldn't invert modelview matrix.")
                    .transpose();

                let normal_matrix_in_view_space = Matrix3::from_cols(
                    normalized_modelview_matrix.x.truncate(),
                    normalized_modelview_matrix.y.truncate(),
                    normalized_modelview_matrix.z.truncate(),
                );
                self.shader_program.set_mat3fv(
                    c_str!("normalMatrixView"),
                    &normal_matrix_in_view_space
                );

                // diffuse map
                gl::ActiveTexture(gl::TEXTURE0);
                gl::BindTexture(gl::TEXTURE_2D, self.material.diffuse_map_texture_id);
                self.shader_program.set_int(c_str!("material.diffuseColor"), 0);

                // specular map
                gl::ActiveTexture(gl::TEXTURE1);
                gl::BindTexture(gl::TEXTURE_2D, self.material.specular_map_texture_id);
                self.shader_program.set_int(c_str!("material.specularColor"), 1);

                if let Some(emission_map_id) = self.material.emission_map_texture_id {
                    gl::ActiveTexture(gl::TEXTURE2);
                    gl::BindTexture(gl::TEXTURE_2D, emission_map_id);
                    self.shader_program.set_int(c_str!("material.emissionColor"), 2);
                }

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}

impl Drop for ContainerCube {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
