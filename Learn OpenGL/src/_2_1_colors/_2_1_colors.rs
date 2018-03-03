extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use cgmath::{Point3, Vector3};

use std::sync::mpsc::Receiver;

use super::scene_graph::SceneGraph;
use super::container_cube::ContainerCube;
use super::lamp_cube::LampCube;

const SRC_FOLDER: &'static str = "_2_1_colors";
const WINDOW_NAME: &'static str = "Learn OpenGL Lesson 2.1 Colors";

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));


    let (mut window, events) = glfw.create_window(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        WINDOW_NAME,
        glfw::WindowMode::Windowed,
    ).expect("Failed to create GLFW window.");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // initialize OpenGL function pointers
    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut scene_graph = SceneGraph::new(SCREEN_WIDTH, SCREEN_HEIGHT);
    scene_graph.update_camera(
        Point3::new(0.0, 0.0, 3.0),
        Point3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let light_color = Vector3::new(1.0, 1.0, 1.0);

    let cube_positions: Vec<Vector3<f32>> = vec![
        Vector3::new(-0.8,  -0.5, -0.5),
    ];
    let object_color = Vector3::new(1.0, 0.5, 0.31);
    scene_graph.add_element(Box::new(
        ContainerCube::new(SRC_FOLDER, cube_positions, object_color, light_color)
    ));

    let lamp_position: Vector3<f32> = Vector3::new(1.2, 1.0, -2.0);
    scene_graph.add_element(Box::new(LampCube::new(SRC_FOLDER, lamp_position)));

    let mut t = 0;
    let scene_graph = scene_graph;

    while !window.should_close() {
        process_events(&mut window, &events);

        // TODO: this would be better tied to a time measurement instead of being a simple frame
        // counter
        t += 1;

        scene_graph.render_frame(t);

        window.swap_buffers();
        glfw.poll_events();
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
