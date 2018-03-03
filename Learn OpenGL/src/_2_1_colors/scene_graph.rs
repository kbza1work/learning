use cgmath::{Deg, Matrix4, Point3, perspective, Vector3};
use cgmath::prelude::*;

extern crate gl;

use super::scene_element::SceneElement;

pub struct SceneGraph {
    view_matrix: Matrix4<f32>,
    projection_matrix: Matrix4<f32>,

    elements: Vec<Box<SceneElement>>,
}

impl SceneGraph {
    pub fn new(screen_width: u32, screen_height: u32) -> Self {
        // should this update when the window's resized?
        let projection_matrix: Matrix4<f32> =
            perspective(Deg(45.0), screen_width as f32 / screen_height as f32, 0.1, 100.0);

        SceneGraph {
            view_matrix: Matrix4::identity(),
            projection_matrix: projection_matrix,
            elements: Vec::new(),
        }
    }

    pub fn update_camera(&mut self, eye: Point3<f32>, center: Point3<f32>, up: Vector3<f32>) {
        self.view_matrix = Matrix4::look_at(eye, center, up);
    }

    pub fn add_element(&mut self, element: Box<SceneElement>) {
        self.elements.push(element);
    }

    pub fn render_frame(&self, t: u32) {

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        for element in self.elements.iter() {
            element.render_frame(t, &self.view_matrix, &self.projection_matrix);
        }
    }
}
