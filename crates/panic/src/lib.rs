fn panic_in_wasm_backtrace(value: usize) {
    if value % 2 == 0 {
        panic!("Hello, world!");
    }
}

#[no_mangle]
pub fn _start() {
    let a = 1 + 1;
    let b = a * 3;

    panic_in_wasm_backtrace(b);
}
