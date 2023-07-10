mod utils;

use curve25519_dalek::edwards::CompressedEdwardsY;
use curve25519_dalek::edwards::EdwardsPoint;
use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::traits::MultiscalarMul;
use js_sys::Error;
use std::convert::TryInto;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn to32(bytes: &[u8]) -> Result<[u8; 32], JsValue> {
    let bytes32: Result<[u8; 32], _> = bytes.try_into();
    Ok(bytes32.map_err(|_| Error::new("invalid scalar size"))?)
}

#[wasm_bindgen]
pub fn scalarMultiply(point_bytes: &[u8], scalar_bytes: &[u8]) -> Result<Vec<u8>, JsValue> {
    let compressed_point = CompressedEdwardsY::from_slice(point_bytes);
    let inpoint = compressed_point
        .decompress()
        .ok_or(Error::new("invalid y coordinate"))?;

    let mut scalar_bytes = to32(scalar_bytes)?;
    scalar_bytes.reverse();
    let scalar = Scalar::from_bytes_mod_order(scalar_bytes);

    let outpoint = EdwardsPoint::multiscalar_mul([scalar], [inpoint]);
    let outpoint_bytes = outpoint.compress().to_bytes();

    Ok(outpoint_bytes.to_vec())
}
