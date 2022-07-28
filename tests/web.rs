//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
extern crate webpackExample;//这里也不明白
use webpackExample::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[cfg(test)]
pub fn input_value() {
}

#[cfg(test)]
pub fn expected_value() {
}

fn main() {

#[wasm_bindgen_test]
pub fn test() {
}
}