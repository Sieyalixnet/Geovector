mod utils;
mod tools;
mod class;
use wasm_bindgen::prelude::*;
extern crate web_sys;
extern crate wee_alloc;
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_export]
macro_rules! log {
       ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}


#[wasm_bindgen]
pub fn test() -> Vec<i64> {
    log!("hello wasm");
    let mut a = Vec::new();
    a.push(10);
    return a;
}
