use cgmath::{Deg, Matrix4, perspective};

extern crate gl;

use common::camera::Camera;

use super::lamps::Lamps;
use super::light::{Light, LightType};
use super::scene_element::SceneElement;

pub struct SceneGraph {
    projection_matrix: Matrix4<f32>,
    pub camera: Camera,

    elements: Vec<Box<SceneElement>>,
    lights: Vec<Light>,
}

impl SceneGraph {
    pub fn new(
        screen_width: u32,
        screen_height: u32,
        src_folder: &str,
        camera: Camera,
        lights: Vec<Light>,
    ) -> Self {
        // TODO: shouldn't this update when the window's resized?
        let projection_matrix: Matrix4<f32> =
            perspective(Deg(45.0), screen_width as f32 / screen_height as f32, 0.1, 100.0);

        SceneGraph {
            projection_matrix: projection_matrix,
            camera: camera,
            elements: vec![],
            lights: lights,
        }
    }

    pub fn add_element(&mut self, element: Box<SceneElement>) {
        self.elements.push(element);
    }

    pub fn render_frame(&mut self, t: f32) {

        self.update_flashlights();

        unsafe {
            gl::ClearColor(0.1, 0.15, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT);
        }

        for element in self.elements.iter() {
            element.render_frame(
                t,
                &self.lights,
                &self.camera.view_matrix(),
                &self.projection_matrix,
            );
        }
    }

    /// update all flashlights to point forward from the camera
    fn update_flashlights(&mut self) {
        let mut flashlights = self.lights.iter_mut().filter(|light| {
            match light.light_type {
                LightType::Spotlight { flashlight, ..} => flashlight,
                _ => false,
            }
        });
        for light in flashlights {
            match light.light_type {
                LightType::Spotlight { ref mut direction, .. } => {
                    *direction = self.camera.front.extend(0.0);
                    light.position = self.camera.position.to_homogeneous();
                },
                _ => {}
            }
        }
    }
}
