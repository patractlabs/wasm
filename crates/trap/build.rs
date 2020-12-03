// build.rs

use std::{env, fs, path::Path, process::Command};

fn panic() -> String {
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

    fs::canonicalize(trap.join("../panic/target/wasm32-unknown-unknown/debug/panic.wasm"))
        .unwrap()
        .to_string_lossy()
        .to_string()
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("panic.rs");
    let wasm = panic();
    fs::write(
        &dest_path,
        format!(
            "\
            ///The WASM binary of panic function \n\
            pub const PANIC_WASM: &str = {:?};\
            ",
            wasm,
        ),
    )
    .unwrap();

    println!("cargo:rerun-if-changed=build.rs");
}
