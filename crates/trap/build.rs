// build.rs

use std::{env, fs, path::Path, process::Command};

fn panic() -> Vec<u8> {
    let manifest = env::var_os("CARGO_MANIFEST_DIR").unwrap();
    let trap = Path::new(&manifest);
    Command::new("cargo")
        .args(vec![
            "build",
            "--target",
            "wasm32-unknown-unknown",
            "--manifest-path",
            "../panic/Cargo.toml",
        ])
        .status()
        .unwrap();

    fs::read(trap.join("../panic/target/wasm32-unknown-unknown/debug/panic.wasm")).unwrap()
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("panic.rs");
    fs::write(
        &dest_path,
        format!(
            "\
            ///The WASM binary of panic function \n\
            pub const PANIC_WASM: &[u8] = &{:?};\
            ",
            &panic(),
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
