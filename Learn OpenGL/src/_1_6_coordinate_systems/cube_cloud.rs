extern crate glfw;

extern crate gl;
use self::gl::types::*;

extern crate image;
use image::GenericImage;

use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::path::Path;

use cgmath::{Deg, Matrix4, Vector3};

use common::shader::Shader;
use super::scene_element::SceneElement;

pub struct CubeCloud {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    box_texture: GLuint,
    smiley_texture: GLuint,
    cube_model_matrices: Vec<Matrix4<f32>>,
}

impl CubeCloud {
    pub fn new(src_folder: &str, cube_positions: Vec<Vector3<f32>>, mix_ratio: f32) -> Self {

        let (shader_program, vao, vbo, box_texture, smiley_texture) =
            CubeCloud::init_opengl(src_folder, mix_ratio);

        let rotation_axis: Vector3<f32> = Vector3::new(1.0, 0.3, 0.5);
        let model_matrices =
            cube_positions
            .iter()
            .enumerate()
            .map(|(index, position)| {
                let translation = Matrix4::from_translation(*position);
                let angle = Deg(20.0) * (index as f32);
                let rotation = Matrix4::from_axis_angle(rotation_axis, angle);

                translation * rotation
            })
            .collect();

        CubeCloud {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            box_texture: box_texture,
            smiley_texture: smiley_texture,
            cube_model_matrices: model_matrices,
        }
    }

    fn init_opengl(src_folder: &str, mix_ratio: f32) -> (Shader, GLuint, GLuint, GLuint, GLuint) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader.glsl", src_folder);
        let box_texture_path = &format!("{}/texture_container.jpg", src_folder);
        let smiley_texture_path = &format!("{}/texture_awesome.png", src_folder);

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

            // texture coordinates
            let offset = 3 * mem::size_of::<GLfloat>();
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
            gl::EnableVertexAttribArray(1);

            gl::BindBuffer(gl::ARRAY_BUFFER, 0);

            let box_texture = CubeCloud::init_texture(box_texture_path, gl::NEAREST, false, gl::RGB);
            let smiley_texture = CubeCloud::init_texture(smiley_texture_path, gl::LINEAR, true, gl::RGBA);

            shader_program.use_program();
            shader_program.set_int(c_str!("texture1"), 0);
            shader_program.set_int(c_str!("texture2"), 1);

            shader_program.set_float(c_str!("mixRatio"), mix_ratio);

            gl::BindVertexArray(0);

            (shader_program, vao, vbo, box_texture, smiley_texture)
        }
    }

    fn init_texture(texture_path: &str, texture_filter: GLenum, flipv: bool, rgb_format: GLuint) -> GLuint {
        let mut texture = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, texture_filter as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, texture_filter as i32);
            let img = image::open(&Path::new(texture_path))
                        .expect("Failed to read texture from file");

            // TODO: take a file type enum and match accordingly

            // needed for png
            let img = if flipv {
                img.flipv() // flip loaded texture on the y-axis.
            } else {
                img
            };
            let data = img.raw_pixels();
            gl::TexImage2D(gl::TEXTURE_2D,
                           0,
                           gl::RGB as i32,
                           img.width() as i32,
                           img.height() as i32,
                           0,
                           rgb_format,
                           gl::UNSIGNED_BYTE,
                           &data[0] as *const u8 as *const c_void);
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }

        texture
    }
}

impl SceneElement for CubeCloud {

    fn render_frame(&self, t: u32, view_matrix: &Matrix4<f32>, projection_matrix: &Matrix4<f32>) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);

            self.shader_program.use_program();
            self.shader_program.set_mat4fv(c_str!("viewMatrix"), view_matrix);
            self.shader_program.set_mat4fv(c_str!("projectionMatrix"), projection_matrix);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.box_texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.smiley_texture);
            gl::BindVertexArray(self.vao);

            for (index, &model_matrix) in self.cube_model_matrices.iter().enumerate() {
                let final_model_matrix = if index % 3 == 0 {
                    model_matrix * Matrix4::from_angle_x(Deg(t as f32))
                } else {
                    model_matrix
                };

                self.shader_program.set_mat4fv(c_str!("modelMatrix"), &final_model_matrix);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}

impl Drop for CubeCloud {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
