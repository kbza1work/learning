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
        file = file.open(file_name);
        let transforms: Vec<Mat4> = Vec::new();

        let scene = Scene::new();

        for each line {
            parse_line(line);
        }
    }
}

fn parse_line(scene: Scene, line: &str, line_number: u32) -> Result<Scene, Vec<SceneReadError>> {
    let line = line without leading whitespace;
    if line length is 0 or line starts with "#" {
        return
    }

    let scene_errors: Vec<SceneReadError> = Vec::new();

    let command = read_until_whitespace_or_newline(line);
    let parameters = rest_of_line(line);

    match command {
        "light" => {
            match parse_light(parameters) {
                Ok(this_light) => scene.lights.push(this_light),
                Err(error) => scene_errors.push(error),
            }
        },

        command => {
            let unrecognized_command_error = SceneError::new(
                format!("Unrecognized command: {}", command),
                line_number,
            );
            scene_errors.push(unrecognized_command_error);
        },
    }

    if scene_errors.is_empty() {
        scene
    } else {
        Err(scene_errors)
    }
}


fn parse_light(parameters) -> Result<Light> {
    if parameters.len() != 8 {
        let error_message = format!(
            "Light command received {} parameters but 8 are required",
            parameters.len()
        );
        return Err(SceneError::new(
            error_message,
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
