use num::{BigInt, Num};
use starknet_crypto;

pub fn pedersen_starknet_crypto_from_hex(x: &str, y: &str) -> String {
    let fx = starknet_crypto::FieldElement::from_hex_be(&x).unwrap();
    let fy = starknet_crypto::FieldElement::from_hex_be(&y).unwrap();

    let result = starknet_crypto::pedersen_hash(&fx, &fy);

    format!("{result:#x}")
}

pub fn pedersen_starknet_crypto_from_dec(x: &str, y: &str) -> String {
    // note: dec to hex transformations in starknet_crypto are really poorly optimized, that's why we use BigInt instead
    let x_hex = BigInt::from_str_radix(x, 10).unwrap().to_str_radix(16);
    let y_hex = BigInt::from_str_radix(y, 10).unwrap().to_str_radix(16);
    let fx = starknet_crypto::FieldElement::from_hex_be(&x_hex).unwrap();
    let fy = starknet_crypto::FieldElement::from_hex_be(&y_hex).unwrap();

    let result = starknet_crypto::pedersen_hash(&fx, &fy);

    // do not use result.to_string() here as it has huge performance impact
    hex_str_to_dec_str(&format!("{result:#x}")[2..])
}

fn hex_str_to_dec_str(hex: &str) -> String {
    BigInt::from_str_radix(hex, 16).unwrap().to_string()
}
