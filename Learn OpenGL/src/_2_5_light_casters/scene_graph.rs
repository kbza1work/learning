use cgmath::{Deg, Matrix4, perspective};

extern crate gl;

use common::camera::Camera;

use super::light::{Light, LightType};
use super::scene_element::SceneElement;

pub struct SceneGraph {
    projection_matrix: Matrix4<f32>,
    pub camera: Camera,

    elements: Vec<Box<SceneElement>>,
    light: Light,
}

impl SceneGraph {
    pub fn new(screen_width: u32, screen_height: u32, camera: Camera, light: Light) -> Self {
        // TODO: shouldn't this update when the window's resized?
        let projection_matrix: Matrix4<f32> =
            perspective(Deg(45.0), screen_width as f32 / screen_height as f32, 0.1, 100.0);

        SceneGraph {
            projection_matrix: projection_matrix,
            camera: camera,
            elements: Vec::new(),
            light: light,
        }
    }

    pub fn add_element(&mut self, element: Box<SceneElement>) {
        self.elements.push(element);
    }

    pub fn render_frame(&mut self, t: f32) {

        // update the flashlight to point forward from the camera
        match self.light.light_type {
            LightType::Spotlight { ref mut direction, inner_angle: _, outer_angle: _ } => {
                *direction = self.camera.front.extend(0.0);
                self.light.position = self.camera.position.to_homogeneous();
            },
            _ => {}
        }

        unsafe {
            gl::ClearColor(0.1, 0.15, 0.15, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // draw some coordinate axes

        for element in self.elements.iter() {
            element.render_frame(
                t,
                &self.light,
                &self.camera.view_matrix(),
                &self.projection_matrix,
            );
        }
    }
}
