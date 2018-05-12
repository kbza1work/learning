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

pub struct Grass {
    standard_shader_program: Shader,
    highlight_shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    model_matrices: Vec<Matrix4<f32>>,
    texture_id: u32,
}

impl Grass {
    pub fn new(src_folder: &str) -> Self {

        let (shader_program, vao, vbo) = Grass::init_opengl(src_folder);

        let cube_positions: Vec<Vector3<f32>> = vec![
            Vector3::new( 0.0,  3.0,  0.0),
            Vector3::new( 2.0,  8.0, -15.0),
            Vector3::new(-1.5,  0.2, -2.5),
            Vector3::new(-3.8,  1.0, -12.3),
            Vector3::new( 2.4,  2.6, -3.5),
            Vector3::new(-1.7,  6.0, -7.5),
            Vector3::new( 1.3,  1.0, -2.5),
            Vector3::new( 1.5,  5.0, -2.5),
            Vector3::new( 1.5,  3.2, -1.5),
            Vector3::new(-1.3,  4.0, -1.5)
        ];

        let model_matrices = cube_positions.iter().enumerate().map(|(i, position)| {
            // calculate the model matrix for each object and pass it to shader before drawing
            let mut model_matrix: Matrix4<f32> = Matrix4::from_translation(*position);
            let angle = 20.0 * i as f32;
            let axis = Vector3::new(1.0, 0.3, 0.5).normalize();
            model_matrix = model_matrix * Matrix4::from_axis_angle(axis, Deg(angle));
            model_matrix
        }).collect();

        Grass {
            standard_shader_program: standard_shader_program,
            highlight_shader_program: highlight_shader_program,
            vao: vao,
            vbo: vbo,
            model_matrices: model_matrices,
            texture_id: load_texture(&format!("{}/texture_grass.png", src_folder), true),
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, standard_fragment_shader_path);

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

impl SceneElement for Grass {

    fn render_frame(
        &self,
        _t: f32,
        lights: &Vec<Light>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    ) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            self.standard_shader_program.use_program();
            self.standard_shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.standard_shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            gl::BindVertexArray(self.vao);

            for model_matrix in self.model_matrices.iter() {
                self.standard_shader_program.set_mat4fv(c_str!("modelMatrix"), &model_matrix);

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
                self.standard_shader_program.set_mat3fv(
                    c_str!("normalMatrixView"),
                    &normal_matrix_in_view_space
                );

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}

impl Drop for Grass {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
