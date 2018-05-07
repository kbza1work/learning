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
use common::heightmap;

use super::scene_element::SceneElement;
use super::light::{Light, LightType};
use super::material::Material;

pub struct Terrain {
    shader_program: Shader,
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,
    num_elements_to_draw: i32,
    model_matrix: Matrix4<f32>,
    material: Material,
}

impl Terrain {
    pub fn new(src_folder: &str, position: Vector3<f32>) -> Self {

        let (shader_program, vao, vbo, ebo, num_elements_to_draw) = Terrain::init_opengl(src_folder);

        let material = Material {
            ambient_color: Vector3::new(1.0, 0.5, 0.31),
            diffuse_map_texture_id: load_texture(&format!("{}/terrain_diffuse_map.png", src_folder), true),
            specular_map_texture_id: load_texture(&format!("{}/terrain_specular_map.png", src_folder), true),
            emission_map_texture_id: None,
            shininess: 32.0,
        };

        Terrain {
            shader_program: shader_program,
            vao: vao,
            vbo: vbo,
            ebo: ebo,
            num_elements_to_draw: num_elements_to_draw,
            model_matrix: Matrix4::from_translation(position),
            material: material,
        }
    }

    fn init_opengl(src_folder: &str) -> (Shader, GLuint, GLuint, GLuint, i32) {

        let vertex_shader_path = &format!("{}/vertex_shader.glsl", src_folder);
        let fragment_shader_path = &format!("{}/fragment_shader.glsl", src_folder);
        let heightmap_path = &format!("{}/heightmap_1.png", src_folder);

        let (mut vertices, indices) = heightmap::heightmap_data(heightmap_path, true);
        let num_elements_to_draw = indices.len() as i32;
        let shader_program = Shader::new(vertex_shader_path, fragment_shader_path);
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);

        // scale the terrain
        let width = 100.0;
        for (index, mut coordinate) in vertices.iter_mut().enumerate() {
            match index % 3 {
                0 => *coordinate *= width,
                2 => *coordinate *= width,
                1 => *coordinate /= 50.0,
                _ => {},
            }
        }

        let min_y = vertices.iter().enumerate().fold(None::<f32>, |acc, (index, &y)| {
            if index % 3 == 1 && (acc.is_none() || y < acc.unwrap()) {
                Some(y)
            } else {
                acc
            }
        }).expect("This heightmap's terrain has no vertices");
        let max_y = vertices.iter().enumerate().fold(None::<f32>, |acc, (index, &y)| {
            if index % 3 == 1 && (acc.is_none() || y > acc.unwrap()) {
                Some(y)
            } else {
                acc
            }
        }).expect("This heightmap's terrain has no vertices");

        println!("terrain elevation varies between {} and {}", min_y, max_y);

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            assert!(vao != 0, "OpenGL failed to create VAO for Terrain object.");
            gl::GenBuffers(1, &mut vbo);
            assert!(vbo != 0, "OpenGL failed to create VBO for Terrain object.");
            gl::GenBuffers(1, &mut ebo);
            assert!(vao != 0, "OpenGL failed to create EBO for Terrain object.");

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &vertices[0] as *const f32 as *const c_void,
                           gl::STATIC_DRAW);

            // TODO: uncomment when the heightmap loader can calculat surface normals
            let stride = 0;
            // let stride = 3 * mem::size_of::<GLfloat>() as GLsizei;

            // positions
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
            gl::EnableVertexAttribArray(0);

            // TODO: uncomment when the heightmap loader can calculat surface normals
            // surface normals
            // let surface_normals_offset = (3 * mem::size_of::<GLfloat>()) as *const c_void;
            // gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, surface_normals_offset);
            // gl::EnableVertexAttribArray(1);

            // index buffer
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                           &indices[0] as *const i32 as *const c_void,
                           gl::STATIC_DRAW);

            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        (shader_program, vao, vbo, ebo, num_elements_to_draw)
    }
}

impl SceneElement for Terrain {

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

            // material properties
            // TODO: accept these in the constructor, set them on the struct, and pass to the
            // shader in init_opengl()
            self.shader_program.set_3fv(c_str!("material.ambientColor"), &self.material.ambient_color);
            self.shader_program.set_float(c_str!("material.shininess"), self.material.shininess);
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

            gl::BindVertexArray(self.vao);

            self.shader_program.set_mat4fv(c_str!("modelMatrix"), &self.model_matrix);

            let normalized_modelview_matrix =
                (view_matrix * self.model_matrix)
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

            gl::DrawElements(gl::LINES, self.num_elements_to_draw, gl::UNSIGNED_INT, ptr::null());

            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Terrain {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteBuffers(1, &self.ebo);
        }
    }
}
