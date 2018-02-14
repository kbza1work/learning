extern crate glfw;
use self::glfw::{Context, Key, Action};

extern crate gl;

use cgmath::{Point3, Vector3};

use std::sync::mpsc::Receiver;

use scene_graph::SceneGraph;
use cube_cloud::CubeCloud;

const SRC_FOLDER: &'static str = "_1_6_coordinate_systems";
const WINDOW_NAME: &'static str = "Learn OpenGL Lesson 1.6 Coordinate Systems";

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub fn main_1_6() {

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
        Point3::new(0.0, 0.0, -1000.0),
        Vector3::new(0.0, 1.0, 0.0),
    );

    let cube_positions: Vec<Vector3<f32>> = vec![
        Vector3::new( 0.0,  0.0,  0.0),
        Vector3::new( 2.0,  5.0, -15.0),
        Vector3::new(-1.5, -2.2, -2.5),
        Vector3::new(-3.8, -2.0, -12.3),
        Vector3::new( 2.4, -0.4, -3.5),
        Vector3::new(-1.7,  3.0, -7.5),
        Vector3::new( 1.3, -2.0, -2.5),
        Vector3::new( 1.5,  2.0, -2.5),
        Vector3::new( 1.5,  0.2, -1.5),
        Vector3::new(-1.3,  1.0, -1.5),
    ];
    scene_graph.add_element(Box::new(CubeCloud::new(SRC_FOLDER, cube_positions, 0.2)));

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
