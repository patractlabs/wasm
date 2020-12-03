//! Trap wasm
include!(concat!(env!("OUT_DIR"), "/panic.rs"));

fn main() {
    println!("{:?}", PANIC_WASM);
}
