#![allow(non_snake_case)]
use std::ffi::{CString, CStr};
use std::fs::File;
use std::io::Read;
use std::ptr;
use std::str;

use gl;
use gl::types::*;

use cgmath::{Matrix, Matrix3, Matrix4, Vector3};
use cgmath::prelude::*;

pub struct Shader {
    pub ID: u32,
}

/// panics if the shader files can't be found, the shaders fail to compile, or the shader program
/// fails to link
#[allow(dead_code)]
impl Shader {
    pub fn new(vertexPath: &str, fragmentPath: &str) -> Shader {
        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile = File::open(vertexPath).expect(&format!("Failed to open {}", vertexPath));
        let mut fShaderFile = File::open(fragmentPath).expect(&format!("Failed to open {}", fragmentPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, "FRAGMENT");
            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            shader.ID = ID;
        }

        shader
    }

    /// activate the shader
    /// ------------------------------------------------------------------------
    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.ID)
    }

    /// utility uniform functions
    /// ------------------------------------------------------------------------
    pub unsafe fn set_bool(&self, name: &CStr, value: bool) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value as i32);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_int(&self, name: &CStr, value: i32) {
        gl::Uniform1i(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_float(&self, name: &CStr, value: f32) {
        gl::Uniform1f(gl::GetUniformLocation(self.ID, name.as_ptr()), value);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_3fv(&self, name: &CStr, value: &Vector3<f32>) {
        let location = gl::GetUniformLocation(self.ID, name.as_ptr());
        gl::Uniform3fv(location, 1, value.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_3f(&self, name: &CStr, x: f32, y: f32, z: f32) {
        gl::Uniform3f(gl::GetUniformLocation(self.ID, name.as_ptr()), x, y, z);
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat3fv(&self, name: &CStr, mat: &Matrix3<f32>) {
        gl::UniformMatrix3fv(gl::GetUniformLocation(self.ID, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }
    /// ------------------------------------------------------------------------
    pub unsafe fn set_mat4fv(&self, name: &CStr, mat: &Matrix4<f32>) {
        gl::UniformMatrix4fv(gl::GetUniformLocation(self.ID, name.as_ptr()), 1, gl::FALSE, mat.as_ptr());
    }

    /// utility function for checking shader compilation/linking errors.
    /// ------------------------------------------------------------------------
    unsafe fn checkCompileErrors(&self, shader: u32, type_: &str) {
        let error_message_capacity: i32 = 1024;
        let mut success = gl::FALSE as GLint;
        let mut info_log = Vec::with_capacity(error_message_capacity as usize);
        // subtract 1 to skip the trailing null character
        info_log.set_len(error_message_capacity as usize - 1);
        if type_ != "PROGRAM" {
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetShaderInfoLog(
                    shader,
                    error_message_capacity,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!("ERROR::SHADER_COMPILATION_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         String::from_utf8_lossy(&info_log),
                );
            }

        } else {
            gl::GetProgramiv(shader, gl::LINK_STATUS, &mut success);
            if success != gl::TRUE as GLint {
                gl::GetProgramInfoLog(
                    shader,
                    error_message_capacity,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                panic!("ERROR::PROGRAM_LINKING_ERROR of type: {}\n{}\n \
                          -- --------------------------------------------------- -- ",
                         type_,
                         String::from_utf8_lossy(&info_log),
                );
            }
        }

    }

    /// Only used in 4.9 Geometry shaders - ignore until then (shader.h in original C++)
    pub fn with_geometry_shader(vertexPath: &str, fragmentPath: &str, geometryPath: &str) -> Self {
        let mut shader = Shader { ID: 0 };
        // 1. retrieve the vertex/fragment source code from filesystem
        let mut vShaderFile = File::open(vertexPath).expect(&format!("Failed to open {}", vertexPath));
        let mut fShaderFile = File::open(fragmentPath).expect(&format!("Failed to open {}", fragmentPath));
        let mut gShaderFile = File::open(geometryPath).expect(&format!("Failed to open {}", geometryPath));
        let mut vertexCode = String::new();
        let mut fragmentCode = String::new();
        let mut geometryCode = String::new();
        vShaderFile
            .read_to_string(&mut vertexCode)
            .expect("Failed to read vertex shader");
        fShaderFile
            .read_to_string(&mut fragmentCode)
            .expect("Failed to read fragment shader");
        gShaderFile
            .read_to_string(&mut geometryCode)
            .expect("Failed to read geometry shader");

        let vShaderCode = CString::new(vertexCode.as_bytes()).unwrap();
        let fShaderCode = CString::new(fragmentCode.as_bytes()).unwrap();
        let gShaderCode = CString::new(geometryCode.as_bytes()).unwrap();

        // 2. compile shaders
        unsafe {
            // vertex shader
            let vertex = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(vertex, 1, &vShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(vertex);
            shader.checkCompileErrors(vertex, "VERTEX");
            // fragment Shader
            let fragment = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(fragment, 1, &fShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(fragment);
            shader.checkCompileErrors(fragment, "FRAGMENT");
            // geometry shader
            let geometry = gl::CreateShader(gl::GEOMETRY_SHADER);
            gl::ShaderSource(geometry, 1, &gShaderCode.as_ptr(), ptr::null());
            gl::CompileShader(geometry);
            shader.checkCompileErrors(geometry, "GEOMETRY");

            // shader Program
            let ID = gl::CreateProgram();
            gl::AttachShader(ID, vertex);
            gl::AttachShader(ID, fragment);
            gl::AttachShader(ID, geometry);
            gl::LinkProgram(ID);
            shader.checkCompileErrors(ID, "PROGRAM");
            // delete the shaders as they're linked into our program now and no longer necessary
            gl::DeleteShader(vertex);
            gl::DeleteShader(fragment);
            gl::DeleteShader(geometry);
            shader.ID = ID;
        }

        shader
    }
}
