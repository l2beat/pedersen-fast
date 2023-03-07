<p align="center">
  <img src="https://em-content.zobj.net/thumbs/240/apple/325/dashing-away_1f4a8.png" width="120" alt="ts-essentials">
  <h2 align="center">pedersen-fast</h1>
  <p align="center">Blazing fast Pedersen hash implementation for Node.JS</p>
  <p align="center"><small>
    Exposes <a href="https://github.com/xJonathanLEI/starknet-rs">starknet-crypto</a>'s implementation written in Rust as WASM package.
    </small>
  </p>
</p>

## Usage

```sh
npm i pedersen-fast
```

Package exposes two functions, one for calculations from HEX strings, the other from decimal strings:

```typescript
import { pedersen_from_hex, pedersen_from_dec } from "pedersen-fast";

// returns a hex string starting with 0x
pedersen_from_hex(
  "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcde",
  "0x11223344556677889900aabbccddeeff11223344556677889900aabbccddeef",
);
```

## Current state

According to our benchmarks, the fastest implementation would be the native module (via NEON). The problem with native modules is a need to compile them on the client or precompile them before. None of these solutions is trivial, that's why we use WASM for now.

`napi-rs` seems to be great at simplifying shipping native modules for nodejs. but its performance is much worse than neons. This needs further research.

## Benchmarks

For `10 000` hash calculations from hex strings:

| Implementation                  | Time      | Hash/s             |
| ------------------------------- | --------- | ------------------ |
| Starknet JS                     | 23,493 ms | 425.659 hash/s     |
| starknet_crypto (WASM)          | 241 ms    | 41,493.776 hash/s  |
| starknet_crypto (native module) | 90 ms     | 111,111.111 hash/s |

## Acknowledgments

Thanks goes to:

- [Herodotus team for their work on benchmarks](https://github.com/HerodotusDev/pedersen-wasm)

* [starknet-rs](https://github.com/xJonathanLEI/starknet-rs) team for the fastest implementation our there :)

## Development

To compile every package and run benchmarks do:

```sh
./scripts/benchmark.sh
```

## License

MIT
