use std::fmt::Display;
use cgmath::{Matrix4, Point3, Vector3};

pub struct Scene {
    lights: Vec<Light>,
    models: Vec<Model>,
    eye: Point3,
    center: Point3,
    up: Vector3,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            lights: Vec::new(),
            models: Vec::new(),
            eye: Point3::new::<f32>(0.0, 0.0, 5.0),
            center: Point3::new::<f32>(0.0, 0.0, 0.0),
            up: Vector3::new::<f32>(0.0, 1.0, 0.0),
        }
    }

    pub fn new_from_file(file_name: &str) -> Result<Scene> {
        let file = try!(File::open(file_name));
        let file = BufReader::new(file);

        let scene = Scene::new();
        let transforms: Vec<Mat4> = Vec::new();

        // the scene files are organized so that some lines set state that modifies successive
        // lines (e.g. the material properties of a model are declares on lines preceding the line
        // on which the model is declared)
        struct CurrentState {
            transforms: Vec<Mat4>,
            ambient: [f32; 4];
            diffuse: [f32; 4];
            specular: [f32; 4];
            emission: [f32; 4];
            shininess: f32;
            size: [f32; 2];
        }

        let state = CurrentState {
            transforms: Vec::new(),
            ambient: [0; 4],
            diffuse: [0; 4],
            specular: [0; 4],
            emission: [0; 4],
            shininess: [0; 4],
            size: [0; 4],
        }

        let mut line_number = 1;
        for line in file.lines() {
            match parse_line(scene, state, line, line_number) {
                Ok(updated_state) => {
                    let (scene, state) = updated_state;
                },
                Err(error) => {
                    return Err(SceneLoadError::new(
                        error.description(),
                        file_name,
                        line_number,
                    ));
                },
            }

            line_number += 1;
        }
    }
}

fn parse_line(
    scene: Scene,
    state: CurrentState,
    line: &str,
    line_number: u32
) -> Result<(Scene, state)> {
    let chunks = line.split_whitespace();
    let command = match chunks.next() {
        Some("#") => return Ok(scene),
        Some(command) => command,
        None => return Ok(scene),
    }
    let parameters: Vec<&str> = chunks.collect();

    match command {
        "light" => {
            let new_light = try!(parse_light(parameters));
            scene.lights.push(new_light);
        },
        "ambient" => {
            state.ambient = try!(parse_material_property(parameters));
        },
        "diffuse" => {
            state.diffuse = try!(parse_material_property(parameters));
        },
        "specular" => {
            state.specular = try!(parse_material_property(parameters));
        },
        "emission" => {
            state.emission = try!(parse_material_property(parameters));
        },
        "shininess" => {
            state.shininess = try!(parse_shininess(parameters));
        },
        "size" => {
            state.size = try!(parse_size(parameters));
        },
        "camera" => {
            (scene.eye, scene.camera, scene.up) = try!(parse_camera_data(parameters));
        },
        command => {
            return Err(Error::new(
                format!("Unrecognized command: {}", command),
            ));
        },
    }

    scene
}


fn parse_light(parameters: Vec<&str>) -> Result<Light> {
    if parameters.len() != 8 {
        let error_message = format!(
            "Light command received {} parameters but 8 are required",
            parameters.len()
        );
        return Err(SceneLoadError::new(
            error_message,
            line,
            line_number,
        ));
    }

    let light_pos = Vec4::new::<f32>(parameters[0..3]);
    let light_color = Vec4::new::<f32>(parameters[4..7]);

    let transformed_light_pos = match transforms.last() {
        Some(transform) => transform * light_pos,
        None => light_pos,
    };

    Light::new(transformed_light_pos, light_color)
}

fn parse_material_property(parameters: Vec<&str>) -> Result<[f32; 4]> {
}

fn parse_shininess(parameters: Vec<&str>) -> Result<[f32; 4]> {
}

fn parse_camera_data(parameters: Vec<&str>) -> Result<(Point3, Point3, Vector3)> {
}

struct SceneLoadError {
    message: String,
}

#[derive(Debug)]
impl SceneLoadError {
    pub fn new(
        message: &str,
        scene_file_name: &str,
        line_number: u32
    ) -> Self {
        let error_message = format!(
            "Error in scene file {}, line {}: {}",
            scene_file_name,
            line_number,
            message,
        );

        Self {
            message: error_message,
        }
    }
}

impl Display for SceneLoadError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, self.message)
    }
}

impl Error for SceneLoadError {
    fn description(&self) -> &str {
        &*self.message
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}
