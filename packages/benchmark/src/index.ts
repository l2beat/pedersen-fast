import { benchmark } from "./benchmark";
import { candidatesDec, candidatesHex } from "./candidates";
import { toHex, randomUint32 } from "./utils";

export async function main() {
  console.log("Running benchmarks...");

  const iterations = [100, 1_000, 10_000];

  console.log("=== HEX strings benchmarks");
  iterations.forEach((i) => benchmark(i, candidatesHex, () => [toHex(randomUint32()), toHex(randomUint32())]));

  console.log("=== DEC strings benchmarks");
  iterations.forEach((i) => benchmark(i, candidatesDec, () => [randomUint32().toString(), randomUint32().toString()]));

  console.log("Done");
}

main().catch((e) => {
  console.error("Error occured: ", e);
  process.exit(1);
});
