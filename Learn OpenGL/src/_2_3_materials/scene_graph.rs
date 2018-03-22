use cgmath::{Deg, Matrix4, perspective};

extern crate gl;

use cgmath::Vector3;

use common::camera::Camera;

use super::light::Light;
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

    pub fn render_frame(&self, t: f32) {

        unsafe {
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        // uncomment this for a constant white light...

        // let current_light = Light {
        //     position: Matrix4::from_angle_z(Deg(100.0 * t)) * self.light.position,
        //     ambient_color: self.light.ambient_color,
        //     diffuse_color: self.light.diffuse_color,
        //     specular_color: self.light.specular_color,
        // };

        // ...or uncomment this for a multicolored light that varies over time
        let current_light_color = Vector3::new((t * 2.0).sin(), (t * 0.7).sin(), (t * 1.3).sin());
        let current_light = Light {
            position: Matrix4::from_angle_z(Deg(100.0 * t)) * self.light.position,
            ambient_color: 0.2 * current_light_color,
            diffuse_color: 0.5 * current_light_color,
            specular_color: self.light.specular_color,
        };

        // draw some coordinate axes

        for element in self.elements.iter() {
            element.render_frame(
                t,
                &current_light,
                &self.camera.view_matrix(),
                &self.projection_matrix,
            );
        }
    }
}
