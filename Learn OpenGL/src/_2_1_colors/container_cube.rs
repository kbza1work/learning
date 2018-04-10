extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::{Matrix4, Vector3};

use common::shader::Shader;
use super::scene_element::SceneElement;

pub struct ContainerCube {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    cube_model_matrices: Vec<Matrix4<f32>>,
    color: Vector3<f32>,
    light_color: Vector3<f32>,
}

impl ContainerCube {
    pub fn new(
        src_folder: &str,
        cube_positions: Vec<Vector3<f32>>,
        color: Vector3<f32>,
        light_color: Vector3<f32>
    ) -> Self {

        let (shader_program, vao, vbo) =
            ContainerCube::init_opengl(src_folder);

        let model_matrices =
            cube_positions
            .iter()
            .map(|position| { Matrix4::from_translation(*position) })
            .collect();

        ContainerCube {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            cube_model_matrices: model_matrices,
            color: color,
            light_color: light_color,
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);

            let vertices: [f32; 180] = [
                // positions       // texture coordinates
                -0.5, -0.5, -0.5,  0.0, 0.0,
                 0.5, -0.5, -0.5,  1.0, 0.0,
                 0.5,  0.5, -0.5,  1.0, 1.0,
                 0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5,  0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 0.0,

                -0.5, -0.5,  0.5,  0.0, 0.0,
                 0.5, -0.5,  0.5,  1.0, 0.0,
                 0.5,  0.5,  0.5,  1.0, 1.0,
                 0.5,  0.5,  0.5,  1.0, 1.0,
                -0.5,  0.5,  0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,

                -0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5, -0.5,  1.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5,  0.5,  1.0, 0.0,

                 0.5,  0.5,  0.5,  1.0, 0.0,
                 0.5,  0.5, -0.5,  1.0, 1.0,
                 0.5, -0.5, -0.5,  0.0, 1.0,
                 0.5, -0.5, -0.5,  0.0, 1.0,
                 0.5, -0.5,  0.5,  0.0, 0.0,
                 0.5,  0.5,  0.5,  1.0, 0.0,

                -0.5, -0.5, -0.5,  0.0, 1.0,
                 0.5, -0.5, -0.5,  1.0, 1.0,
                 0.5, -0.5,  0.5,  1.0, 0.0,
                 0.5, -0.5,  0.5,  1.0, 0.0,
                -0.5, -0.5,  0.5,  0.0, 0.0,
                -0.5, -0.5, -0.5,  0.0, 1.0,

                -0.5,  0.5, -0.5,  0.0, 1.0,
                 0.5,  0.5, -0.5,  1.0, 1.0,
                 0.5,  0.5,  0.5,  1.0, 0.0,
                 0.5,  0.5,  0.5,  1.0, 0.0,
                -0.5,  0.5,  0.5,  0.0, 0.0,
                -0.5,  0.5, -0.5,  0.0, 1.0
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

            let stride = 5 * mem::size_of::<GLfloat>() as GLsizei;

            // positions
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            (shader_program, vao, vbo)
        }
    }
}

impl SceneElement for ContainerCube {

    fn render_frame(&self, _t: u32, view_matrix: &Matrix4<f32>, projection_matrix: &Matrix4<f32>) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);
            self.shader_program.set_3fv(c_str!("objectColor"), &self.color);
            self.shader_program.set_3fv(c_str!("lightColor"), &self.light_color);

            gl::BindVertexArray(self.vao);

            for model_matrix in self.cube_model_matrices.iter() {
                self.shader_program.set_mat4fv(c_str!("modelMatrix"), &model_matrix);
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
