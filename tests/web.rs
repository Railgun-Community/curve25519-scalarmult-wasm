//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use curve25519_scalarmult_wasm::scalarMultiply;
use js_sys::Error;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn expected_outcome() -> Result<(), String> {
    let point_hex = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad, 0xbe, 0xef, 0xde, 0xad,
        0xbe, 0xef,
    ];
    let scalar_hex = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x91,
        0x57, 0xcc, 0x50, 0x12, 0xde, 0x59, 0x15, 0x68, 0x08, 0x0f, 0xf8, 0x10, 0x81, 0x02, 0xdd,
        0xef, 0xa9,
    ];

    let expected = [
        0xdb, 0x93, 0xe6, 0x8c, 0x1b, 0xa1, 0x21, 0x28, 0x8d, 0x9a, 0xe8, 0x6d, 0x10, 0x3f, 0x8a,
        0xb3, 0xa1, 0x54, 0x52, 0x1f, 0x8c, 0xe9, 0xdd, 0x34, 0xc5, 0x02, 0xfc, 0x00, 0x15, 0xf9,
        0x0f, 0xa2,
    ];

    let actual = scalarMultiply(&point_hex, &scalar_hex);
    if actual.is_err() {
        return Err(format!("Error: {:?}", actual.err().unwrap()));
    } else {
        assert_eq!(actual.unwrap(), expected);
    }
    return Ok(());
}

#[wasm_bindgen_test]
fn throws_invalid_y_coordinate() -> Result<(), String> {
    let point_hex = [
        122, 247, 122, 242, 41, 199, 22, 160, 168, 36, 83, 200, 250, 170, 208, 189, 116, 82, 157,
        77, 82, 192, 120, 42, 62, 13, 148, 15, 17, 141, 227, 22,
    ];
    let scalar_hex = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x8a, 0x91,
        0x57, 0xcc, 0x50, 0x12, 0xde, 0x59, 0x15, 0x68, 0x08, 0x0f, 0xf8, 0x10, 0x81, 0x02, 0xdd,
        0xef, 0xa9,
    ];

    let actual = scalarMultiply(&point_hex, &scalar_hex);
    if actual.is_err() {
        let x: Error = actual.err().unwrap().into();
        assert_eq!(x.message(), "invalid y coordinate");
        return Ok(());
    } else {
        return Err(String::from("Expected error in this case"));
    }
}
