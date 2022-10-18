mod utils;

use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

fn to32(bytes: &[u8]) -> [u8; 32] {
    bytes
        .try_into()
        .expect("slice with incorrect length, expected 32")
}

#[wasm_bindgen]
pub fn scalarmult(point_bytes: &[u8], scalar_bytes: &[u8]) -> Vec<u8> {
    let compressed_point = CompressedEdwardsY::from_slice(point_bytes);
    let inpoint = compressed_point.decompress().unwrap();

    let mut scalar_bytes = to32(scalar_bytes);
    scalar_bytes.reverse();
    let scalar = Scalar::from_bytes_mod_order(scalar_bytes);

    let outpoint = EdwardsPoint::multiscalar_mul([scalar], [inpoint]);
    let outpoint_bytes = outpoint.compress().to_bytes();

    outpoint_bytes.to_vec()
}
