use pedersen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn pedersen_from_hex(x: &str, y: &str) -> String {
    pedersen::pedersen_starknet_crypto_from_hex(x, y)
}

#[wasm_bindgen]
pub fn pedersen_from_dec(x: &str, y: &str) -> String {
    pedersen::pedersen_starknet_crypto_from_dec(x, y)
}
