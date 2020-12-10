//! Trap wasm
include!(concat!(env!("OUT_DIR"), "/panic.rs"));

use std::fs::File;
use wasmi::Module;

fn load_from_file(filename: &str) -> Module {
    use std::io::prelude::*;
    let mut file = File::open(filename).unwrap();
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).unwrap();
    Module::from_buffer(buf).unwrap()
}

fn main() {
    // let module = ::deserialize_file(&PANIC_WASM)
    //     .unwrap()
    //     .parse_names()
    //     .unwrap();
    let module = load_from_file(&PANIC_WASM);
    // .module
    // .parse_names()
    // .expect("Parse names failed");

    // println!("has name section: {:?}", module.has_names_section());
    //
    // let names_section = module.names_section();
    // println!(
    //     "names_section: {:#?}",
    //     names_section.unwrap().functions().unwrap().names()
    // );
    let instance = wasmi::ModuleInstance::new(&module, &wasmi::ImportsBuilder::default())
        .expect("Failed to instantiate module")
        .run_start(&mut wasmi::NopExternals)
        .expect("Failed to run start");

    instance
        .invoke_export("_start", &[], &mut wasmi::NopExternals)
        .expect("Failed");
}
