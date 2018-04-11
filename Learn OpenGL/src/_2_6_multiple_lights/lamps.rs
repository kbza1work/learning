extern crate glfw;

extern crate gl;
use self::gl::types::*;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::Matrix4;

use common::shader::Shader;
use super::scene_element::SceneElement;
use super::light::{Light, LightType};

pub struct Lamps {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
}

impl Lamps {
    pub fn new(src_folder: &str) -> Self {

        let (shader_program, vao, vbo) =
            Lamps::init_opengl(src_folder);

        Lamps {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader_lamp.glsl", src_folder);

        unsafe {
            let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);

            let vertices: [f32; 180] = [
                // positions       // texture coordinates
                -0.2, -0.2, -0.2,  0.0, 0.0,
                 0.2, -0.2, -0.2,  1.0, 0.0,
                 0.2,  0.2, -0.2,  1.0, 1.0,
                 0.2,  0.2, -0.2,  1.0, 1.0,
                -0.2,  0.2, -0.2,  0.0, 1.0,
                -0.2, -0.2, -0.2,  0.0, 0.0,

                -0.2, -0.2,  0.2,  0.0, 0.0,
                 0.2, -0.2,  0.2,  1.0, 0.0,
                 0.2,  0.2,  0.2,  1.0, 1.0,
                 0.2,  0.2,  0.2,  1.0, 1.0,
                -0.2,  0.2,  0.2,  0.0, 1.0,
                -0.2, -0.2,  0.2,  0.0, 0.0,

                -0.2,  0.2,  0.2,  1.0, 0.0,
                -0.2,  0.2, -0.2,  1.0, 1.0,
                -0.2, -0.2, -0.2,  0.0, 1.0,
                -0.2, -0.2, -0.2,  0.0, 1.0,
                -0.2, -0.2,  0.2,  0.0, 0.0,
                -0.2,  0.2,  0.2,  1.0, 0.0,

                 0.2,  0.2,  0.2,  1.0, 0.0,
                 0.2,  0.2, -0.2,  1.0, 1.0,
                 0.2, -0.2, -0.2,  0.0, 1.0,
                 0.2, -0.2, -0.2,  0.0, 1.0,
                 0.2, -0.2,  0.2,  0.0, 0.0,
                 0.2,  0.2,  0.2,  1.0, 0.0,

                -0.2, -0.2, -0.2,  0.0, 1.0,
                 0.2, -0.2, -0.2,  1.0, 1.0,
                 0.2, -0.2,  0.2,  1.0, 0.0,
                 0.2, -0.2,  0.2,  1.0, 0.0,
                -0.2, -0.2,  0.2,  0.0, 0.0,
                -0.2, -0.2, -0.2,  0.0, 1.0,

                -0.2,  0.2, -0.2,  0.0, 1.0,
                 0.2,  0.2, -0.2,  1.0, 1.0,
                 0.2,  0.2,  0.2,  1.0, 0.0,
                 0.2,  0.2,  0.2,  1.0, 0.0,
                -0.2,  0.2,  0.2,  0.0, 0.0,
                -0.2,  0.2, -0.2,  0.0, 1.0
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

impl SceneElement for Lamps {

    fn render_frame(
        &self,
        _t: f32,
        lights: &Vec<Light>,
        view_matrix: &Matrix4<f32>,
        projection_matrix: &Matrix4<f32>,
    ) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            gl::BindVertexArray(self.vao);

            let point_lights = lights.iter().filter(|light| {
                match light.light_type {
                    LightType::Point { .. } => true,
                    _ => false,
                }
            });
            for light in point_lights {
                self.shader_program.set_3fv(c_str!("light.ambientColor"), &light.ambient_color);
                self.shader_program.set_3fv(c_str!("light.diffuseColor"), &light.diffuse_color);
                self.shader_program.set_3fv(c_str!("light.specularColor"), &light.specular_color);

                let model_matrix = Matrix4::from_translation(light.position.truncate());
                self.shader_program.set_mat4fv(c_str!("modelMatrix"), &model_matrix);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}

impl Drop for Lamps {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}

