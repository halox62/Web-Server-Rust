// src/lib.rs
#[no_mangle]
pub extern "C" fn run() { 
    println!("Hello from plugin!");
}