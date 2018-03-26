extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

extern crate image;
use image::GenericImage;

use std::sync::mpsc::Receiver;
use std::ptr;
use std::mem;
use std::os::raw::c_void;
use std::ffi::CStr;
use std::path::Path;

use cgmath::{Matrix4, vec3,  Rad};
use cgmath::prelude::*;

use common::shader::Shader;
use common::texture::load_texture;

// settings
const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

const SRC_FOLDER: &'static str = "_1_5_transformations";

pub fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Learn OpenGL Lesson 1.5 Transformations",
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (shader_program, vao, vbo, ebo, texture1, texture2) = unsafe {
        let shader_program = Shader::new(
            &format!("{}/vertex_shader.glsl", SRC_FOLDER),
            &format!("{}/fragment_shader.glsl", SRC_FOLDER),
        );

        let vertices: [f32; 32] = [
            // positions       // colors        // texture coords
             0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0, // top right
             0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0, // bottom right
            -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0, // bottom left
            -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0  // top left
        ];
        let indices = [
            0, 1, 3,  // first Triangle
            1, 2, 3   // second Triangle
        ];
        let (mut vao, mut vbo, mut ebo) = (0, 0, 0);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (indices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &indices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        let stride = 8 * mem::size_of::<GLfloat>() as GLsizei;

        // position attribute
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, stride, ptr::null());
        gl::EnableVertexAttribArray(0);

        // color attribute
        let offset = 3 * mem::size_of::<GLfloat>();
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
        gl::EnableVertexAttribArray(1);

        // texture coordinate attribute
        let offset = 6 * mem::size_of::<GLfloat>();
        gl::VertexAttribPointer(2, 2, gl::FLOAT, gl::FALSE, stride, offset as *const c_void);
        gl::EnableVertexAttribArray(2);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // load and create a texture
        let mut texture1 = 0;
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(&format!("{}/texture_container.jpg", SRC_FOLDER)))
                    .expect("Failed to texture from file");
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGB,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // load and create a texture
        let mut texture2 = 0;
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2); // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
        // set the texture wrapping parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32); // set texture wrapping to gl::REPEAT (default wrapping method)
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        // set texture filtering parameters
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(&format!("{}/texture_awesome.png", SRC_FOLDER)))
                    .expect("Failed to load texture from file");
        let img = img.flipv(); // flip loaded texture on the y-axis.
        let data = img.raw_pixels();
        gl::TexImage2D(gl::TEXTURE_2D,
                       0,
                       gl::RGB as i32,
                       img.width() as i32,
                       img.height() as i32,
                       0,
                       gl::RGBA,
                       gl::UNSIGNED_BYTE,
                       &data[0] as *const u8 as *const c_void);
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader_program.use_program();
        shader_program.set_int(c_str!("texture1"), 0);
        shader_program.set_int(c_str!("texture2"), 1);

        gl::BindVertexArray(0);

        (shader_program, vao, vbo, ebo, texture1, texture2)
    };

    while !window.should_close() {
        process_events(&mut window, &events);

        unsafe {
            let mix_ratio = 0.2;

            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            shader_program.use_program();
            shader_program.set_float(c_str!("mixRatio"), mix_ratio);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);
            gl::BindVertexArray(vao);

            let mut model_view_matrix: Matrix4<f32> = Matrix4::identity();

            shader_program.set_mat4fv(c_str!("modelViewMatrix"), &model_view_matrix);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

            model_view_matrix =
                model_view_matrix * Matrix4::<f32>::from_translation(vec3(-0.5, 0.5, 0.0));
            model_view_matrix =
                model_view_matrix * Matrix4::<f32>::from_angle_z(Rad(glfw.get_time() as f32));
            shader_program.set_mat4fv(c_str!("modelViewMatrix"), &model_view_matrix);

            shader_program.set_mat4fv(c_str!("modelViewMatrix"), &model_view_matrix);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());

        }

        window.swap_buffers();
        glfw.poll_events();
    }

    // de-allocate all resources once they've outlived their purpose:
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteBuffers(1, &ebo);
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // window resize event
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
