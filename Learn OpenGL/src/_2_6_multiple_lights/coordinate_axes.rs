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
use super::light::Light;

pub struct CoordinateAxes {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,

    x_color: Vector3<f32>,
    y_color: Vector3<f32>,
    z_color: Vector3<f32>,
}

impl CoordinateAxes {
    pub fn new(
        src_folder: &str,
        x_color: Vector3<f32>,
        y_color: Vector3<f32>,
        z_color: Vector3<f32>
    ) -> Self {

        let (shader_program, vao, vbo) =
            CoordinateAxes::init_opengl(src_folder);

        CoordinateAxes {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            x_color: x_color,
            y_color: y_color,
            z_color: z_color,
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/coordinate_axes_vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/coordinate_axes_fragment_shader.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);

            let vertices: [f32; 18] = [
                -10000.0,  0.0,  0.0,
                 10000.0,  0.0,  0.0,
                 0.0, -10000.0,  0.0,
                 0.0,  10000.0,  0.0,
                 0.0,  0.0, -10000.0,
                 0.0,  0.0,  10000.0,
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

            let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;

            // positions
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            gl::BindVertexArray(0);

            (shader_program, vao, vbo)
        }
    }
}

impl SceneElement for CoordinateAxes {

    fn render_frame(
        &self,
        _t: f32,
        _lights: &Vec<Light>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    ) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            gl::BindVertexArray(self.vao);

            self.shader_program.set_3fv(c_str!("color"), &self.x_color);
            gl::DrawArrays(gl::LINES, 0, 2);
            self.shader_program.set_3fv(c_str!("color"), &self.y_color);
            gl::DrawArrays(gl::LINES, 2, 2);
            self.shader_program.set_3fv(c_str!("color"), &self.z_color);
            gl::DrawArrays(gl::LINES, 4, 2);
        }
    }
}

impl Drop for CoordinateAxes {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
