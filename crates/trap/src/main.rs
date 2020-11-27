//! Trap wasm
include!(concat!(env!("OUT_DIR"), "/panic.rs"));

fn main() {
    let module = parity_wasm::deserialize_file(&PANIC_WASM)
        .unwrap()
        .parse_names()
        .unwrap();

    println!("has name section: {:?}", module.has_names_section());

    let names_section = module.names_section();
    println!(
        "names_section: {:#?}",
        names_section.unwrap().functions().unwrap().names()
    );
}
