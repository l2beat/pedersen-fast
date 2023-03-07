export function randomUint32(): BigInt {
  return BigInt(Math.floor(Math.random() * 4294967295));
}

export function toHex(num: BigInt): string {
  return "0x" + num.toString(16);
}
