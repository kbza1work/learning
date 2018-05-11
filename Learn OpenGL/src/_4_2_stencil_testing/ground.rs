extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::{Matrix4, Vector3};

use common::shader::Shader;
use common::texture::load_texture;
use super::scene_element::SceneElement;
use super::light::Light;

pub struct Ground {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    model_matrix: Matrix4<f32>,
    texture_id: u32,
}

impl Ground {
    pub fn new(src_folder: &str) -> Self {

        let (shader_program, vao, vbo) =
            Ground::init_opengl(src_folder);

        let model_matrix =
            Matrix4::from_nonuniform_scale(1_000.0, 0.0, 1_000.0) *
            Matrix4::from_translation(Vector3::new(0.0, 0.5, 0.0));

        Ground {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            model_matrix: model_matrix,
            texture_id: load_texture(&format!("{}/texture_marble.jpg", src_folder), true),
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/texturing_vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/texturing_fragment_shader.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);

            let vertices: [f32; 30] = [
                // positions         // texture coordinates
                 5.0, -0.5,  5.0,    1_000.0, 0.0,
                -5.0, -0.5,  5.0,    0.0, 0.0,
                -5.0, -0.5, -5.0,    0.0, 1_000.0,

                 5.0, -0.5,  5.0,    1_000.0, 0.0,
                -5.0, -0.5, -5.0,    0.0, 1_000.0,
                 5.0, -0.5, -5.0,    1_000.0, 1_000.0,
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

            // texture coordinates
            let texture_coords_offset = (3 * mem::size_of::<GLfloat>()) as *const c_void;
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, texture_coords_offset);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);

            (shader_program, vao, vbo)
        }
    }
}

impl SceneElement for Ground {

    fn render_frame(
        &self,
        _t: f32,
        _lights: &Vec<Light>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    ) {
        unsafe {
            gl::StencilMask(0x00); // disable writing to the stencil buffer
            gl::Enable(gl::DEPTH_TEST);

            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("modelMatrix"), &self.model_matrix);
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.texture_id);
            gl::BindVertexArray(self.vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl Drop for Ground {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
