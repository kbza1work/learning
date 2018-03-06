extern crate gl;
extern crate image;
extern crate cgmath;
extern crate tobj;

#[macro_use]
mod common;

mod _1_1_hello_window;
mod _1_2_hello_triangle;
mod _1_3_shaders;
mod _1_4_textures;
mod _1_5_transformations;
mod _1_6_coordinate_systems;

mod _2_1_colors;
mod _2_2_basic_lighting;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1.1` for 1.1-Hello Window");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        "1.1" => _1_1_hello_window::main(),
        "1.2" => _1_2_hello_triangle::main(),
        "1.3" => _1_3_shaders::main(),
        "1.4" => _1_4_textures::main(),
        "1.5" => _1_5_transformations::main(),
        "1.6" => _1_6_coordinate_systems::main(),

        "2.1" => _2_1_colors::main(),
        "2.2" => _2_2_basic_lighting::main(),

        _ => println!("Unknown tutorial ID"),
    }
}
