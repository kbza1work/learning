extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::prelude::*;
use cgmath::{Deg, Matrix3, Matrix4, Vector3};

use common::shader::Shader;
use common::texture::load_texture;

use super::scene_element::SceneElement;
use super::light::{Light, LightType};
use super::material::Material;

pub struct ContainerCube {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    model_matrices: Vec<Matrix4<f32>>,
    material: Material,
}

impl ContainerCube {
    pub fn new(src_folder: &str) -> Self {

        let (shader_program, vao, vbo) = ContainerCube::init_opengl(src_folder);

        let cube_positions: Vec<Vector3<f32>> = vec![
            Vector3::new( 0.0,  0.0,  0.0),
            Vector3::new( 2.0,  5.0, -15.0),
            Vector3::new(-1.5, -2.2, -2.5),
            Vector3::new(-3.8, -2.0, -12.3),
            Vector3::new( 2.4, -0.4, -3.5),
            Vector3::new(-1.7,  3.0, -7.5),
            Vector3::new( 1.3, -2.0, -2.5),
            Vector3::new( 1.5,  2.0, -2.5),
            Vector3::new( 1.5,  0.2, -1.5),
            Vector3::new(-1.3,  1.0, -1.5)
        ];

        let model_matrices = cube_positions.iter().enumerate().map(|(i, position)| {
            // calculate the model matrix for each object and pass it to shader before drawing
            let mut model_matrix: Matrix4<f32> = Matrix4::from_translation(*position);
            let angle = 20.0 * i as f32;
            // don't forget to normalize the axis!
            let axis = Vector3::new(1.0, 0.3, 0.5).normalize();
            model_matrix = model_matrix * Matrix4::from_axis_angle(axis, Deg(angle));
            model_matrix
        }).collect();

        let material = Material {
            ambient_color: Vector3::new(1.0, 0.5, 0.31),
            diffuse_map_texture_id: load_texture(&format!("{}/texture_container.png", src_folder), true),
            specular_map_texture_id: load_texture(&format!("{}/texture_container_specular_map.png", src_folder), true),
            emission_map_texture_id: None,
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
        let fragment_shader_path = &format!("{}/fragment_shader_spotlight.glsl", src_folder);

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
            gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, texture_coords_offset);
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

            match light.light_type {
                LightType::Point {
                    constant,
                    linear,
                    quadratic,
                } => {
                    self.shader_program.set_float(c_str!("light.constant"), constant);
                    self.shader_program.set_float(c_str!("light.linear"), linear);
                    self.shader_program.set_float(c_str!("light.quadratic"), quadratic);
                },
                LightType::Directional => {},
                LightType::Spotlight {
                    direction,
                    inner_angle,
                    outer_angle,
                } => {
                    self.shader_program.set_3fv(c_str!("light.directionView"), &(view_matrix * direction).truncate());
                    self.shader_program.set_float(c_str!("light.cutOffInner"), inner_angle.cos());
                    self.shader_program.set_float(c_str!("light.cutOffOuter"), outer_angle.cos());
                },
            }

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

                match self.material.emission_map_texture_id {
                    Some(emission_map_id) => {
                        gl::ActiveTexture(gl::TEXTURE2);
                        gl::BindTexture(gl::TEXTURE_2D, emission_map_id);
                        self.shader_program.set_bool(c_str!("material.emissionPresent"), true);
                        self.shader_program.set_int(c_str!("material.emissionColor"), 2);
                    },
                    None => {
                        self.shader_program.set_bool(c_str!("material.emissionPresent"), false);
                    }
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
