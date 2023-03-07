use crate::constants::*;

use ark_ec::{AffineCurve, ProjectiveCurve};
use ark_ff::BigInteger;
use ark_ff::PrimeField;
use starknet_curve::{Affine, Fq, Fr};

struct Constants {
    // bits
    pub low_part_bits: u32,

    // points
    pub hash_shift_point: Affine,
    pub p0: Affine,
    pub p1: Affine,
    pub p2: Affine,
    pub p3: Affine,
}

lazy_static! {
    static ref HASH_SHIFT_POINT: Affine = Affine::new(HASH_SHIFT_POINT_X, HASH_SHIFT_POINT_Y, false);
    static ref P0_STATIC: Affine = Affine::new(P0_STATIC_X, P0_STATIC_Y, false);
    static ref P1_STATIC: Affine = Affine::new(P1_STATIC_X, P1_STATIC_Y, false);
    static ref P2_STATIC: Affine = Affine::new(P2_STATIC_X, P2_STATIC_Y, false);
    static ref P3_STATIC: Affine = Affine::new(P3_STATIC_X, P3_STATIC_Y, false);

    static ref CONSTANTS: Constants = Constants {
        // bits
        low_part_bits: LOW_PART_BITS,
        // points
        hash_shift_point: *HASH_SHIFT_POINT,
        p0: *P0_STATIC,
        p1: *P1_STATIC,
        p2: *P2_STATIC,
        p3: *P3_STATIC,
    };
}

fn process_single_element(element: Fq, p1: Affine, p2: Affine) -> Affine {
    let mut high_nibble = element.into_repr();
    high_nibble.divn(CONSTANTS.low_part_bits);

    // element will be array of 32 bytes, each element 8 bits
    // so if we put first byte to zero we ensure that the next 248 bits are unchanged
    // and first 8 bits are zero, which is same as performing: low_part = element & low_part_mask
    let mut bytes = element.into_repr().to_bytes_be();
    bytes[0] = 0u8;
    let low_part = Fr::from_be_bytes_mod_order(&bytes);

    (p1.mul(low_part) + p2.mul(high_nibble)).into_affine()
}

/// Computes the Starkware version of the Pedersen hash of x and y.
/// The hash is defined by:
/// shift_point + x_low * P_0 + x_high * P1 + y_low * P2  + y_high * P3
/// where x_low is the 248 low bits of x, x_high is the 4 high bits of x and similarly for y.
/// shift_point, P_0, P_1, P_2, P_3 are constant points generated from the digits of pi.
pub fn geo_pedersen_hash(x: &Fq, y: &Fq) -> Fq {
    let point = CONSTANTS.hash_shift_point
        + process_single_element(*x, CONSTANTS.p0, CONSTANTS.p1)
        + process_single_element(*y, CONSTANTS.p2, CONSTANTS.p3);

    point.x
}

/// Computes a hash chain over the data, in the following order:
///     h(h(h(h(0, data[0]), data[1]), ...), data[n-1]), n).
/// The hash is initialized with 0 and ends with the data length appended.
/// The length is appended in order to avoid collisions of the following kind:
/// H([x,y,z]) = h(h(x,y),z) = H([w, z]) where w = h(x,y).
// fn compute_hash_on_elements(data: &Vec<Fq>) -> Result<Fq, Error> {
//     if data.len() == 0 {
//         return Err(Error::EmptyDataError);
//     }

//     let mut acc = Fq::zero();
//     let data_len = Fq::from(data.len() as u64);
//     for y in data.iter().chain(std::iter::once(&data_len)) {
//         acc = geo_pedersen_hash(&acc, y);
//     }

//     Ok(acc)
// }

#[cfg(test)]
mod tests {
    use super::geo_pedersen_hash;
    use ark_ff::field_new;
    use starknet_curve::Fq;

    #[test]
    fn test_pedersen_with_cairo() {
        // CAIRO: pedersen_hash(17, 71) -> 1785999660572583615240258164082465668299482253941125073628479392605449162275

        let seventeen = Fq::from(17u64);
        let seventy_one = Fq::from(71u64);

        let expected = field_new!(
            Fq,
            "1785999660572583615240258164082465668299482253941125073628479392605449162275"
        );

        let pedersen_h = geo_pedersen_hash(&seventeen, &seventy_one);
        assert_eq!(expected, pedersen_h)
    }
}
