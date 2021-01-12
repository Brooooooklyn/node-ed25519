# `@napi-rs/ed25519`

![https://github.com/Brooooooklyn/node-ed25519/actions](https://github.com/Brooooooklyn/node-ed25519/workflows/CI/badge.svg)

> [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) binding for NodeJS.

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
    38 490 ops/s, ±2.31%   | fastest

  nan:
    5 284 ops/s, ±1.97%    | slowest, 86.27% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Running "sign" suite...
Progress: 100%

  napi:
    22 279 ops/s, ±1.48%   | fastest

  nan:
    13 300 ops/s, ±1.10%   | slowest, 40.3% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Running "sign" suite...
Progress: 100%

  napi:
    22 698 ops/s, ±0.45%   | fastest

  nan:
    13 408 ops/s, ±0.44%   | slowest, 40.93% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Running "verify" suite...
Progress: 100%

  napi:
    16 373 ops/s, ±0.39%   | fastest

  nan:
    8 052 ops/s, ±0.42%    | slowest, 50.82% slower

Finished 2 cases!
  Fastest: napi
  Slowest: nan
Done in 44.19s.
```

## Support matrix

|                 | node10 | node12 | node14 | node15 |
| --------------- | ------ | ------ | ------ | ------ |
| Windows x64     | ✓      | ✓      | ✓      | ✓      |
| macOS x64/arm64 | ✓      | ✓      | ✓      | ✓      |
| Linux x64 gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux x64 musl  | ✓      | ✓      | ✓      | ✓      |
| Linux arm gnu   | ✓      | ✓      | ✓      | ✓      |
| Linux arm64 gnu | ✓      | ✓      | ✓      | ✓      |
| Android arm64   | ✓      | ✓      | ✓      | ✓      |
