#![allow(non_upper_case_globals)]
extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;
use self::gl::types::*;

use std::sync::mpsc::Receiver;
use std::ffi::CString;
use std::ptr;
use std::str;
use std::mem;
use std::os::raw::c_void;

// settings
const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;

const vertexShaderSource: &str = r#"
    #version 330 core
    layout (location = 0) in vec3 aPos;
    void main() {
       gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
    }
"#;

const orangeShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
    }
"#;

const violetShaderSource: &str = r#"
    #version 330 core
    out vec4 FragColor;
    void main() {
       FragColor = vec4(0.3f, 0.3f, 1.0f, 1.0f);
    }
"#;

#[allow(non_snake_case)]
pub fn main_1_2() {
    // glfw: initialize and configure
    // ------------------------------
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw.create_window(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "Learn OpenGL Lesson 1.2 Hello Triangle",
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // load OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let (orangeShaderProgram, violetShaderProgram, orangeVAO, violetVAO) = unsafe {
        // vertex shader
        let vertexShader = gl::CreateShader(gl::VERTEX_SHADER);
        let c_str_vert = CString::new(vertexShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(vertexShader, 1, &c_str_vert.as_ptr(), ptr::null());
        gl::CompileShader(vertexShader);

        // check for shader compile errors
        let mut success = gl::FALSE as GLint;
        let mut infoLog = Vec::with_capacity(512);
        infoLog.set_len(512 - 1); // subtract 1 to skip the trailing null character
        gl::GetShaderiv(vertexShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(vertexShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("Vertex shader compilation error:\n{}", str::from_utf8(&infoLog).unwrap());
        }

        // fragment shaders

        let orangeShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(orangeShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(orangeShader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(orangeShader);
        // check for shader compile errors
        gl::GetShaderiv(orangeShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(orangeShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("Fragment shader compilation error:\n{}", str::from_utf8(&infoLog).unwrap());
        }

        let violetShader = gl::CreateShader(gl::FRAGMENT_SHADER);
        let c_str_frag = CString::new(violetShaderSource.as_bytes()).unwrap();
        gl::ShaderSource(violetShader, 1, &c_str_frag.as_ptr(), ptr::null());
        gl::CompileShader(violetShader);
        // check for shader compile errors
        gl::GetShaderiv(violetShader, gl::COMPILE_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetShaderInfoLog(violetShader, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("Fragment shader compilation error:\n{}", str::from_utf8(&infoLog).unwrap());
        }

        // link shaders

        let orangeShaderProgram = gl::CreateProgram();
        gl::AttachShader(orangeShaderProgram, vertexShader);
        gl::AttachShader(orangeShaderProgram, orangeShader);
        gl::LinkProgram(orangeShaderProgram);
        // check for linking errors
        gl::GetProgramiv(orangeShaderProgram, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(orangeShaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("Shader program linking error:\n{}", str::from_utf8(&infoLog).unwrap());
        }
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(orangeShader);

        let violetShaderProgram = gl::CreateProgram();
        gl::AttachShader(violetShaderProgram, vertexShader);
        gl::AttachShader(violetShaderProgram, violetShader);
        gl::LinkProgram(violetShaderProgram);
        // check for linking errors
        gl::GetProgramiv(violetShaderProgram, gl::LINK_STATUS, &mut success);
        if success != gl::TRUE as GLint {
            gl::GetProgramInfoLog(violetShaderProgram, 512, ptr::null_mut(), infoLog.as_mut_ptr() as *mut GLchar);
            println!("Shader program linking error:\n{}", str::from_utf8(&infoLog).unwrap());
        }
        gl::DeleteShader(vertexShader);
        gl::DeleteShader(violetShader);

        let vertices: [f32; 12] = [
             0.5,  0.5, 0.0,  // top right
             0.5, -0.5, 0.0,  // bottom right
            -0.5, -0.5, 0.0,  // bottom left
            -0.5,  0.5, 0.0,  // top left
        ];
        let orangeIndices = [
            0, 1, 3,
        ];
        let violetIndices = [
            1, 2, 3,
        ];
        let (mut VBO, mut orangeVAO, mut violetVAO, mut orangeEBO, mut violetEBO) = (0, 0, 0, 0, 0);
        gl::GenVertexArrays(1, &mut orangeVAO);
        gl::GenVertexArrays(1, &mut violetVAO);
        gl::GenBuffers(1, &mut VBO);
        gl::GenBuffers(1, &mut orangeEBO);
        gl::GenBuffers(1, &mut violetEBO);

        // bind the Vertex Array Object first, then bind and set vertex buffer(s), and then configure vertex attributes(s).
        gl::BindVertexArray(orangeVAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, orangeEBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (orangeIndices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &orangeIndices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);


        gl::BindVertexArray(violetVAO);

        gl::BindBuffer(gl::ARRAY_BUFFER, VBO);
        gl::BufferData(gl::ARRAY_BUFFER,
                       (vertices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &vertices[0] as *const f32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, violetEBO);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                       (violetIndices.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                       &violetIndices[0] as *const i32 as *const c_void,
                       gl::STATIC_DRAW);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * mem::size_of::<GLfloat>() as GLsizei, ptr::null());
        gl::EnableVertexAttribArray(0);

        // note that this is allowed, the call to gl::VertexAttribPointer registered VBO as the vertex attribute's bound vertex buffer object so afterwards we can safely unbind
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);

        // You can unbind the VAO afterwards so other VAO calls won't accidentally modify this VAO, but this rarely happens. Modifying other
        // VAOs requires a call to glBindVertexArray anyways so we generally don't unbind VAOs (nor VBOs) when it's not directly necessary.
        gl::BindVertexArray(0);

        // uncomment this call to draw in wireframe polygons.
        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);

        (orangeShaderProgram, violetShaderProgram, orangeVAO, violetVAO)
    };

    // render loop
    // -----------
    while !window.should_close() {
        // events
        // -----
        process_events(&mut window, &events);

        // render
        // ------
        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            gl::BindVertexArray(orangeVAO);
            gl::UseProgram(orangeShaderProgram);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());

            gl::BindVertexArray(violetVAO);
            gl::UseProgram(violetShaderProgram);
            gl::DrawElements(gl::TRIANGLES, 3, gl::UNSIGNED_INT, ptr::null());
        }

        // glfw: swap buffers and poll IO events (keys pressed/released, mouse moved etc.)
        // -------------------------------------------------------------------------------
        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_events(window: &mut glfw::Window, events: &Receiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            // window resize
            glfw::WindowEvent::FramebufferSize(width, height) => {
                unsafe { gl::Viewport(0, 0, width, height) }
            }
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
            _ => {}
        }
    }
}
