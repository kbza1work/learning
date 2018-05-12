extern crate glfw;
use self::glfw::Context;

extern crate gl;

use cgmath::{Deg, Point3, Vector3, Vector4};

use common::camera::{Camera, Pitch, Yaw};
use common::input::*;

use super::coordinate_axes::CoordinateAxes;
use super::scene_graph::SceneGraph;
use super::container_cube::ContainerCube;
use super::light::{Light, LightType};

const SRC_FOLDER: &'static str = "_2_5_light_casters";
const WINDOW_NAME: &'static str = "Learn OpenGL Lesson 2.5 Light Casters";

const SCREEN_WIDTH: u32 = 1024;
const SCREEN_HEIGHT: u32 = 768;

pub fn main() {

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

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

    let lamp_position = Vector3::new(0.0, 0.0, 15.0);
    let light = Light {
        position: lamp_position.extend(1.0),
        ambient_color: Vector3::new(0.2, 0.2, 0.2),
        diffuse_color: Vector3::new(1.0, 1.0, 1.0),
        specular_color: Vector3::new(1.0, 1.0, 1.0),
        // light_type: LightType::Point {
        //     constant: 1.0,
        //     linear: 0.045,
        //     quadratic: 0.0075,
        // },
        light_type: LightType::Spotlight {
            direction: Vector4::new(1.0, 0.0, 0.0, 0.0),
            inner_angle: Deg(6.0),
            outer_angle: Deg(9.0),
        },
    };

    let camera = Camera::new(
        Point3::new(0.0, 0.0, 10.0),
        Vector3::unit_y(),
        Yaw::new(-90.0),
        Pitch::new(0.0),
    );
    let mut scene_graph = SceneGraph::new(SCREEN_WIDTH, SCREEN_HEIGHT, camera, light);

    scene_graph.add_element(Box::new(
        CoordinateAxes::new(
            SRC_FOLDER,
            Vector3::new(1.0, 0.0, 0.0),
            Vector3::new(0.0, 1.0, 0.0),
            Vector3::new(0.0, 0.0, 1.0),
        )
    ));

    scene_graph.add_element(Box::new(ContainerCube::new(SRC_FOLDER)));

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