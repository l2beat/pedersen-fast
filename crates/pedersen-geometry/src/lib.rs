use std::str::FromStr;

use ark_ff::{Fp256, PrimeField};
use leftpad::left_pad_char;
use num::{BigInt, Num};
use pedersen::geo_pedersen_hash;
use starknet_curve::{Fq, FqParameters};

#[macro_use]
extern crate lazy_static;

mod constants;
mod pedersen;

pub fn pedersen_geometry_from_hex(x: &str, y: &str) -> String {
    let fx = field_element_from_be_hex(&x);
    let fy = field_element_from_be_hex(&y);

    let result = geo_pedersen_hash(&fx, &fy);

    fp_to_hex_string(&result)
}

pub fn pedersen_geometry_from_dec(x: &str, y: &str) -> String {
    let fx = Fq::from_str(&x).unwrap();
    let fy = Fq::from_str(&y).unwrap();

    let result = geo_pedersen_hash(&fx, &fy);

    hex_str_to_dec_str(&result.into_repr().to_string().trim_start_matches('0'))
}

fn field_element_from_be_hex(hex: &str) -> Fq {
    // here we need to deal with odd length hex strings
    let hex_padded = &left_pad_char(&hex[2..], 10, '0')[0..10];
    let decoded = hex::decode(hex_padded).unwrap();

    if decoded.len() > 32 {
        panic!("hex string too long");
    }

    Fq::from_be_bytes_mod_order(&decoded)
}

fn fp_to_hex_string(fp: &Fp256<FqParameters>) -> String {
    let mut formatted_hash = String::from("0x");
    formatted_hash.push_str(&fp.into_repr().to_string().trim_start_matches('0'));
    formatted_hash.to_lowercase()
}

fn hex_str_to_dec_str(hex: &str) -> String {
    BigInt::from_str_radix(hex, 16).unwrap().to_string()
}
