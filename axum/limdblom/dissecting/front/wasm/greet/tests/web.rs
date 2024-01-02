//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use greet::sum;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]

fn test_sum() {
    assert_eq!(sum(2, 2), 4);
}
