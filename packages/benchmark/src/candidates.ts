import * as starknet from "starknet";
const native = require("../../native");
const wasm = require("../../wasm");

export interface Candidate {
  name: string;
  execute: (input: [string, string]) => string;
}

export const candidatesHex: Candidate[] = [
  {
    name: "JS - starknet.js",
    execute: ([a, b]) => starknet.hash.pedersen([a, b]),
  },
  {
    name: "Rust WASM - starknet_crypto",
    execute: ([a, b]) => wasm.pedersen_starknet_crypto_from_hex(a, b),
  },
  {
    name: "Rust Native - starknet_crypto",
    execute: ([a, b]) => native.pedersen_starknet_crypto_from_hex(a, b),
  },
  {
    name: "Rust WASM - Geo",
    execute: ([a, b]) => wasm.pedersen_geometry_from_hex(a, b),
  },
  {
    name: "Rust Native - Geo",
    execute: ([a, b]) => native.pedersen_geometry_from_hex(a, b),
  },
];

export const candidatesDec: Candidate[] = [
  {
    name: "JS - starknet.js",
    // starknet js always returns HEX strings so we need to convert them back to DEC string
    execute: ([a, b]) => BigInt(starknet.hash.pedersen([a, b])).toString(10),
  },
  {
    name: "Rust WASM - starknet_crypto",
    execute: ([a, b]) => wasm.pedersen_starknet_crypto_from_dec(a, b),
  },
  {
    name: "Rust Native - starknet_crypto",
    execute: ([a, b]) => native.pedersen_starknet_crypto_from_dec(a, b),
  },
  {
    name: "Rust WASM - Geo",
    execute: ([a, b]) => wasm.pedersen_geometry_from_dec(a, b),
  },
  {
    name: "Rust Native - Geo",
    execute: ([a, b]) => native.pedersen_geometry_from_dec(a, b),
  },
];
