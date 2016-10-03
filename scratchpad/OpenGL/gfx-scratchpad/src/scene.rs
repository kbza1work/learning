use light::Light;
use model::Model;

use std::fmt::{self, Display};
use std::fs::File;
use std::io::{BufRead, BufReader};

use errors::*;
use cgmath::{Matrix4, Point3, Vector3, Vector4};


#[derive(Debug, PartialEq)]
pub struct Scene {
    lights: Vec<Light>,
    models: Vec<Model>,
    eye: Point3<f32>,
    center: Point3<f32>,
    up: Vector3<f32>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            lights: Vec::new(),
            models: Vec::new(),
            eye: Point3::new(0.0, 0.0, 5.0),
            center: Point3::new(0.0, 0.0, 0.0),
            up: Vector3::new(0.0, 1.0, 0.0),
        }
    }

    pub fn new_from_file(file_name: &str) -> Result<Scene> {
        let file = try!(File::open(file_name).chain_err(|| {
            format!("Couldn't open scene file: {}", file_name)
        }));
        let file = BufReader::new(file);

        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let mut line_number = 1;

        for line_result in file.lines() {
            match line_result {
                Ok(line) => {
                    let (scene, state) = try!(parse_line(scene, state, &*line).chain_err(|| {
                        format!("Error in scene file {} on line {}", file_name, line_number)
                    }));
                },
                Err(error) => {
                    error.chain_err(|| {
                        format!(
                            "Couldn't read line {} of scene file {}",
                            line_number,
                            file_name,
                        )
                    })
                }
            }
            line_number += 1;
        }

        Ok(scene)
    }
}

fn parse_line(
    scene: Scene,
    state: SceneLoaderState,
    line: &str,
) -> Result<(Scene, SceneLoaderState)> {
    let chunks = line.split_whitespace();
    let command = match chunks.next() {
        Some("#") => return Ok((scene, state)),
        Some(command) => command,
        None => return Ok((scene, state)),
    };
    let parameters: Vec<&str> = chunks.collect();

    match command {
        "light" => {
            let new_light = try!(parse_light(parameters, &state).chain_err(|| {
                "Error parsing light definition"
            }));
            scene.lights.push(new_light);
        },
        "ambient" => {
            state.ambient = try!(parse_material_property(parameters).chain_err(|| {
                "Error parsing ambient definition"
            }));
        },
        "diffuse" => {
            state.diffuse = try!(parse_material_property(parameters).chain_err(|| {
                "Error parsing diffuse definition"
            }));
        },
        "specular" => {
            state.specular = try!(parse_material_property(parameters).chain_err(|| {
                "Error parsing specular definition"
            }));
        },
        "emission" => {
            state.emission = try!(parse_material_property(parameters).chain_err(|| {
                "Error parsing emission definition"
            }));
        },
        "shininess" => {
            state.shininess = try!(parse_shininess(parameters).chain_err(|| {
                "Error parsing shininess definition"
            }));
        },
        "size" => {
            state.size = try!(parse_size(parameters).chain_err(|| {
                "Error parsing size definition"
            }));
        },
        "camera" => {
            let (eye, center, up) = try!(parse_camera_data(parameters)).chain_err(|| {
                "Error parsing camera definition"
            });
            scene.eye = eye;
            scene.center = center;
            scene.up = up;
        },
        command => {
            return Err(Error::new(
                format!("Unrecognized command: {}", command),
            ));
        },
    }

    Ok((scene, state))
}


fn parse_light(parameters: Vec<&str>, state: &SceneLoaderState) -> Result<Light> {
    if parameters.len() != 8 {
        return Err(Error::new(format!(
            "light command received {} parameters but 8 are required",
            parameters.len()
        )));
    }

    let parameters_results: Vec<Result<f32>> = parameters.iter().map(|numeric_string| {
        try!(numeric_string.parse::<f32>().or_else(|_| {
            let integer = try!(numeric_string.parse::<i32>().chain_err(|| {
                format!("Couldn't parse {}", numeric_string)
            }));
            Ok(integer as f32)
        }).chain_err(|| {
            format!("Couldn't parse {}", numeric_string)
        })
    }).collect();
    if let Some((index, &result)) = parameters_results.iter().enumerate().find(|&(index, &result)| {
        result.is_err()
    }) {
        let invalid_parameter = parameters.get(index).unwrap();
        try!(result.chain_err(|| {
            format!(
                "couldn't parse {} into a numeric value",
                invalid_parameter,
            )
        }));
    }
    let parameters: Vec<f32> = parameters_results.into_iter().map(|result| {
        result.unwrap()
    }).collect();
    let parameters_iterator = parameters.into_iter();

    let light_pos_iterator = parameters_iterator.take(4);
    let light_pos = Vector4::new(
        light_pos_iterator.next().unwrap(),
        light_pos_iterator.next().unwrap(),
        light_pos_iterator.next().unwrap(),
        light_pos_iterator.next().unwrap(),
    );

    let transformed_light_pos = match state.transforms.last() {
        Some(transform) => *transform * light_pos,
        None => light_pos,
    };

    let light_color_iterator = parameters_iterator.take(4);
    let light_color: [f32; 4] = [
        light_color_iterator.next().unwrap(),
        light_color_iterator.next().unwrap(),
        light_color_iterator.next().unwrap(),
        light_color_iterator.next().unwrap(),
    ];

    Ok(Light::new(transformed_light_pos, light_color))
}

fn parse_material_property(parameters: Vec<&str>) -> Result<[f32; 4]> {
}

fn parse_shininess(parameters: Vec<&str>) -> Result<f32> {
}

fn parse_camera_data(parameters: Vec<&str>) -> Result<(Point3<f32>, Point3<f32>, Vector3<f32>)> {
}

fn parse_size(parameters: Vec<&str>) -> Result<(u32, u32)> {
}

#[derive(Debug)]
pub struct SceneLoadError {
    message: String,
}

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

        SceneLoadError {
            message: error_message,
        }
    }
}

impl Display for SceneLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl ::std::error::Error for SceneLoadError {
    fn description(&self) -> &str {
        &*self.message
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}

#[derive(Debug)]
struct LowLevelSceneLoadError {
    message: String,
}

impl LowLevelSceneLoadError {
    pub fn new(error_message: String) -> Self {
        LowLevelSceneLoadError {
            message: error_message,
        }
    }
}

impl Display for LowLevelSceneLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl ::std::error::Error for LowLevelSceneLoadError {
    fn description(&self) -> &str {
        &*self.message
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        None
    }
}

// the scene files are organized so that some lines set state that modifies successive
// lines (e.g. the material properties of a model are declares on lines preceding the line
// on which the model is declared)
struct SceneLoaderState {
    transforms: Vec<Matrix4<f32>>,
    ambient: [f32; 4],
    diffuse: [f32; 4],
    specular: [f32; 4],
    emission: [f32; 4],
    shininess: f32,
    size: (f32, f32),   // (width, height)
}

impl SceneLoaderState {
    pub fn new() -> Self {
        SceneLoaderState {
            transforms: Vec::new(),
            ambient: [0f32; 4],
            diffuse: [0f32; 4],
            specular: [0f32; 4],
            emission: [0f32; 4],
            shininess: 0f32,
            size: (0f32, 0f32),
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn parse_empty_line() {
        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let line = "";
        let line_number = 54;

        let (updated_scene, updated_state) = parse_line().unwrap();
        assert!(updated_scene == scene);
        assert!(updated_state == state);
    }

    #[test]
    fn parse_comment_line() {
        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let line = " # light 1 3 7";
        let line_number = 26;

        let (updated_scene, updated_state) = parse_line().unwrap();
        assert!(updated_scene == scene);
        assert!(updated_state == state);
    }

    #[test]
    fn parse_whitespace_line() {
        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let line = " \t\t";
        let line_number = 43;

        let (updated_scene, updated_state) = parse_line().unwrap();
        assert!(updated_scene == scene);
        assert!(updated_state == state);
    }

    #[test]
    fn parse_light_line() {
        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let line = "\tlight 0.6 0 0.1 0 1 0.6 0.2 1 ";
        let line_number = 29;

        let (updated_scene, updated_state) = parse_line().unwrap();
        assert!(updated_state == state);
        let mut expected_scene = Scene::new();
        let light = vec!(Light {
            color: [1.0, 0.6, 0.2, 1.0],
            position: [0.6, 0.0, 0.1, 0.0],
        });
        expected_scene.lights = light;
        assert!(updated_scene == expected_scene);
    }

    #[test]
    fn parse_ambient_line() {
        let scene = Scene::new();
        let state = SceneLoaderState::new();
        let line = "\tambient 0.5 0.7 0.9 1 ";
        let line_number = 32;

        let (updated_scene, updated_state) = parse_line().unwrap();
        assert!(updated_scene == scene);
        let mut expected_state = SceneLoaderState::new();
        expected_state.ambient = [0.5, 0.7, 0.9, 1.0];
        assert!(updated_state == expected_state);
    }
}
