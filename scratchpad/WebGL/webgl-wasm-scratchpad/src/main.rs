fn main() {
    println!("Rust main() executed");
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    return a + b
}
