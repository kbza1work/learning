extern crate glfw;
use self::glfw::Context;

extern crate gl;

use std::ptr;
use std::os::raw::c_void;
use std::ffi::CStr;

use cgmath::{Point3, Vector3, Vector4};

use common::camera::{Camera, Pitch, Yaw};
use common::input::*;

use super::coordinate_axes::CoordinateAxes;
use super::scene_graph::SceneGraph;
use super::light::{Light, LightType};
use super::terrain::Terrain;

const SRC_FOLDER: &'static str = "heightmap";
const WINDOW_NAME: &'static str = "Heightmap";

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

extern "system" fn gl_debug_output(source: gl::types::GLenum,
                                 type_: gl::types::GLenum,
                                 id: gl::types::GLuint,
                                 severity: gl::types::GLenum,
                                 _length: gl::types::GLsizei,
                                 message: *const gl::types::GLchar,
                                 _user_param: *mut c_void)
{
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        // ignore these non-significant error codes
        return;
    }

    // may be useful to turn on notfications later
    if severity == gl::DEBUG_SEVERITY_NOTIFICATION {
        return;
    }

    println!("---------------");
    let message = unsafe { CStr::from_ptr(message).to_str().unwrap() };
    println!("Debug message ({}): {}", id, message);
    match source {
        gl::DEBUG_SOURCE_API =>             println!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM =>   println!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => println!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY =>     println!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION =>     println!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER =>           println!("Source: Other"),
        _ =>                                println!("Source: Unknown enum value")
    }

    match type_ {
       gl::DEBUG_TYPE_ERROR =>               println!("Type: Error"),
       gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => println!("Type: Deprecated Behaviour"),
       gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR =>  println!("Type: Undefined Behaviour"),
       gl::DEBUG_TYPE_PORTABILITY =>         println!("Type: Portability"),
       gl::DEBUG_TYPE_PERFORMANCE =>         println!("Type: Performance"),
       gl::DEBUG_TYPE_MARKER =>              println!("Type: Marker"),
       gl::DEBUG_TYPE_PUSH_GROUP =>          println!("Type: Push Group"),
       gl::DEBUG_TYPE_POP_GROUP =>           println!("Type: Pop Group"),
       gl::DEBUG_TYPE_OTHER =>               println!("Type: Other"),
       _ =>                                  println!("Type: Unknown enum value")
    }

    match severity {
       gl::DEBUG_SEVERITY_HIGH =>         println!("Severity: high"),
       gl::DEBUG_SEVERITY_MEDIUM =>       println!("Severity: medium"),
       gl::DEBUG_SEVERITY_LOW =>          println!("Severity: low"),
       gl::DEBUG_SEVERITY_NOTIFICATION => println!("Severity: notification"),
       _ =>                               println!("Severity: Unknown enum value")
    }
}

pub fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    #[cfg(debug_assertions)]
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    let mut first_mouse = true;
    let mut last_x: f32 = SCREEN_WIDTH as f32 / 2.0;
    let mut last_y: f32 = SCREEN_HEIGHT as f32 / 2.0;

    // timing
    let mut delta_time: f32; // time between current frame and last frame
    let mut last_frame: f32 = 0.0;


    let (mut window, events) = glfw.create_window(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WINDOW_NAME,
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window.");

    window.make_current();
    // window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);

    // tell GLFW to capture our mouse
    // window.set_cursor_mode(glfw::CursorMode::Disabled);

    // initialize OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    #[cfg(debug_assertions)]
    unsafe {
        let mut flags = 0;
        gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut flags);
        if flags as u32 & gl::CONTEXT_FLAG_DEBUG_BIT != 0 {
            gl::Enable(gl::DEBUG_OUTPUT);
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS); // makes sure errors are displayed synchronously
            gl::DebugMessageCallback(gl_debug_output, ptr::null());
            gl::DebugMessageControl(gl::DONT_CARE, gl::DONT_CARE, gl::DONT_CARE, 0, ptr::null(), gl::TRUE);
        }
        else {
            println!("Debug Context not active! Check if your driver supports the extension.")
        }
    }

    let lights = vec![
        Light {
            position: Vector4::new(0.0, -1.0, 0.0, 0.0),
            ambient_color: Vector3::new(0.0, 0.0, 0.0),
            diffuse_color: Vector3::new(1.0, 1.0, 1.0),
            specular_color: Vector3::new(1.0, 1.0, 1.0),
            light_type: LightType::Directional,
        },
    ];

    let camera = Camera::new(
        Point3::new(00.0, 20.0, 0.0),
        Vector3::unit_y(),
        Yaw::new(45.0),
        Pitch::new(-10.0),
    );
    let mut scene_graph = SceneGraph::new(
        SCREEN_WIDTH, SCREEN_HEIGHT, SRC_FOLDER, camera, lights
    );

    scene_graph.add_element(Box::new(
        CoordinateAxes::new(
            SRC_FOLDER,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
    ));

    scene_graph.add_element(Box::new(Terrain::new(SRC_FOLDER, Vector3::new(0.0, 0.0, 0.0))));

    let mut scene_graph = scene_graph;

    while !window.should_close() {
        let current_frame = glfw.get_time() as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_events(&events, &mut first_mouse, &mut last_x, &mut last_y, &mut scene_graph.camera);
        process_input(&mut window, delta_time, &mut scene_graph.camera);

        scene_graph.render_frame(current_frame);

        window.swap_buffers();
        glfw.poll_events();
    }
}
