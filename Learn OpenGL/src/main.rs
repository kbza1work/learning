extern crate gl;
extern crate image;
extern crate cgmath;
extern crate tobj;

#[macro_use]
mod common;

#[cfg(feature = "chapter-1")]
mod _1_1_hello_window;
mod _1_2_hello_triangle;
mod _1_3_shaders;
mod _1_4_textures;
#[cfg(feature = "chapter-1")]
use _1_1_hello_window::*;
use _1_2_hello_triangle::*;
use _1_3_shaders::*;
use _1_4_textures::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1.1` for 1.1-Hello Window");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1.1" => main_1_1(),
        #[cfg(feature = "chapter-1")] "1.2" => main_1_2(),
        #[cfg(feature = "chapter-1")] "1.3" => main_1_3(),
        #[cfg(feature = "chapter-1")] "1.4" => main_1_4(),

        _ => println!("Unknown tutorial id"),
    }
}
