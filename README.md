# `@napi-rs/ed25519`

![https://github.com/Brooooooklyn/node-ed25519/actions](https://github.com/Brooooooklyn/node-ed25519/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/ed25519)](https://packagephobia.com/result?p=@napi-rs/ed25519)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/ed25519.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/ed25519?minimal=true)

> [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) binding for Node.js.

## Install this test package

Comparison with [curve25519-n](https://github.com/scottnonnenberg-signal/node-curve25519.git)

```
yarn add @napi-rs/ed25519
```

## Performance

```text
Running "generateKeyPair" suite...
Progress: 100%

  napi:
    38 457 ops/s, ±2.15%   | fastest

  nan:
    5 263 ops/s, ±1.53%    | slowest, 86.31% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Running "sign" suite...
Progress: 100%

  napi:
    22 100 ops/s, ±1.39%   | fastest

  nan:
    13 317 ops/s, ±0.57%   | slowest, 39.74% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Running "verify" suite...
Progress: 100%

  napi:
    16 663 ops/s, ±0.67%   | fastest

  nan:
    7 982 ops/s, ±2.61%    | slowest, 52.1% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Done in 33.10s.
```

## Support matrix

|                  | node12 | node14 | node16 |
| ---------------- | ------ | ------ | ------ |
| Windows x64      | ✓      | ✓      | ✓      |
| Windows x32      | ✓      | ✓      | ✓      |
| Windows arm64    | ✓      | ✓      | ✓      |
| macOS x64        | ✓      | ✓      | ✓      |
| macOS arm64      | ✓      | ✓      | ✓      |
| Linux x64 gnu    | ✓      | ✓      | ✓      |
| Linux x64 musl   | ✓      | ✓      | ✓      |
| Linux arm gnu    | ✓      | ✓      | ✓      |
| Linux arm64 gnu  | ✓      | ✓      | ✓      |
| Linux arm64 musl | ✓      | ✓      | ✓      |
| Android arm64    | ✓      | ✓      | ✓      |
| FreeBSD x64      | ✓      | ✓      | ✓      |

## API

```typescript
export function generateKeyPair(): {
  publicKey: Buffer
  privateKey: Buffer
}

export function sign(privateKey: Buffer, message: Buffer): Buffer

export function verify(publicKey: Buffer, message: Buffer, signature: Buffer): boolean
```
