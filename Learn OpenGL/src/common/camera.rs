use cgmath::{InnerSpace, Matrix4, Point3, Vector3, Zero};

pub enum CameraMovement { Forward, Backward, Left, Right }
use self::CameraMovement::*;

#[derive(Debug)]
pub struct Pitch(f32);
impl Pitch {
    pub fn new(yaw: f32) -> Self {
        Pitch { 0: yaw }
    }
}
impl Default for Pitch {
    fn default() -> Self {
        Pitch(0.0)
    }
}

#[derive(Debug)]
pub struct Yaw(f32);
impl Yaw {
    pub fn new(yaw: f32) -> Self {
        Yaw { 0: yaw }
    }
}
impl Default for Yaw {
    fn default() -> Self {
        Yaw(-90.0)
    }
}

#[derive(Debug)]
pub struct Speed(f32);
impl Speed {
    pub fn new(yaw: f32) -> Self {
        Speed { 0: yaw }
    }
}
impl Default for Speed {
    fn default() -> Self {
        Speed(2.5)
    }
}

#[derive(Debug)]
pub struct MouseSensitivity(f32);
impl Default for MouseSensitivity {
    fn default() -> Self {
        MouseSensitivity(0.1)
    }
}

#[derive(Debug)]
pub struct Zoom(f32);
impl Default for Zoom {
    fn default() -> Self {
        Zoom(45.0)
    }
}

pub struct Camera {
    position: Point3<f32>,
    front: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,
    world_up: Vector3<f32>,
    yaw: f32,
    pitch: f32,

    movement_speed: f32,
    mouse_sensitivity: f32,
    zoom: f32,
}

impl Camera {
    pub fn new(position: Point3<f32>, up: Vector3<f32>, yaw: Yaw, pitch: Pitch) -> Self {
        let mut camera = Camera {
            front: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::zero(),
            up: Vector3::zero(),
            movement_speed: Speed::default().0,
            mouse_sensitivity: MouseSensitivity::default().0,
            zoom: Zoom::default().0,
            position: position,
            world_up: up,
            yaw: yaw.0,
            pitch: pitch.0,
        };

        camera.update_camera_vectors();
        camera
    }

    pub fn view_matrix(&self) -> Matrix4<f32> {
        Matrix4::look_at(self.position, self.position + self.front, self.up)
    }

    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let distance_moved = self.movement_speed * delta_time;
        match direction {
            Forward => self.position += self.front * distance_moved,
            Backward => self.position += -(self.front * distance_moved),
            Right => self.position += self.right * distance_moved,
            Left => self.position += -(self.right * distance_moved),
        }
    }

    pub fn process_mouse_movement(&mut self, mut x_offset: f32, mut y_offset: f32, constrain_pitch: bool) {
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // Make sure that when pitch is out of bounds, screen doesn't get flipped
        if constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            } else if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }

        // update front, right and up Vectors using the new Euler angles
        self.update_camera_vectors();
    }

    pub fn process_scroll(&mut self, y_offset: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= y_offset;
        } else if self.zoom <= 1.0 {
            self.zoom = 1.0;
        } else if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }


    /// Recalculate internal front vector from current Euler angles
    fn update_camera_vectors(&mut self) {
        self.front = Vector3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        ).normalize();
        self.right = self.front.cross(self.world_up).normalize();
        self.up = self.right.cross(self.front).normalize();
    }
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Camera {
            position: Point3::new(0.0, 0.0, 0.0),
            front: Vector3::new(0.0, 0.0, -1.0),
            right: Vector3::zero(),
            up: Vector3::zero(),
            world_up: Vector3::unit_y(),
            movement_speed: Speed::default().0,
            mouse_sensitivity: MouseSensitivity::default().0,
            zoom: Zoom::default().0,
            yaw: Yaw::default().0,
            pitch: Pitch::default().0,
        };

        camera.update_camera_vectors();
        camera
    }
}
