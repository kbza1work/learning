extern crate gl;
extern crate image;
extern crate cgmath;
extern crate tobj;

#[cfg(feature = "chapter-1")]
mod _1_1_hello_window;
#[cfg(feature = "chapter-1")]
use _1_1_hello_window::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1.1` for 1.1-Hello Window");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1.1" => main_1_1(),

        _     => println!("Unknown tutorial id")
    }
}
