# `@napi-rs/ed25519`

![https://github.com/Brooooooklyn/node-ed25519/actions](https://github.com/Brooooooklyn/node-ed25519/workflows/CI/badge.svg)
[![install size](https://packagephobia.com/badge?p=@napi-rs/ed25519)](https://packagephobia.com/result?p=@napi-rs/ed25519)
[![Downloads](https://img.shields.io/npm/dm/@napi-rs/ed25519.svg?sanitize=true)](https://npmcharts.com/compare/@napi-rs/ed25519?minimal=true)

> [ed25519-dalek](https://github.com/dalek-cryptography/ed25519-dalek) binding for Node.js.

## Install

```
yarn add @napi-rs/ed25519
```

## Performance

Compare with [noble-ed25519](https://github.com/paulmillr/noble-ed25519/)

### Generate KeyPair

```text
┌─────────┬────────────────────┬──────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name          │ ops/sec  │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼────────────────────┼──────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@napi-rs/ed25519' │ '84,313' │ 11860.560927010862 │ '±1.19%' │ 42157   │
│ 1       │ '@noble/ed25519'   │ '7,735'  │ 129267.36039296696 │ '±0.67%' │ 3868    │
└─────────┴────────────────────┴──────────┴────────────────────┴──────────┴─────────┘
```

### Sign

```text
┌─────────┬────────────────────┬──────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name          │ ops/sec  │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼────────────────────┼──────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@napi-rs/ed25519' │ '48,326' │ 20692.732246316882 │ '±2.57%' │ 24164   │
│ 1       │ '@noble/ed25519'   │ '4,115'  │ 243012.00291544604 │ '±0.52%' │ 2058    │
└─────────┴────────────────────┴──────────┴────────────────────┴──────────┴─────────┘
```

### Verify

```text
┌─────────┬────────────────────┬──────────┬────────────────────┬──────────┬─────────┐
│ (index) │ Task Name          │ ops/sec  │ Average Time (ns)  │ Margin   │ Samples │
├─────────┼────────────────────┼──────────┼────────────────────┼──────────┼─────────┤
│ 0       │ '@napi-rs/ed25519' │ '34,145' │ 29286.012651555568 │ '±0.13%' │ 17073   │
│ 1       │ '@noble/ed25519'   │ '923'    │ 1083121.3181818079 │ '±0.36%' │ 462     │
└─────────┴────────────────────┴──────────┴────────────────────┴──────────┴─────────┘
```

## API

```typescript
export function generateKeyPair(): {
  publicKey: Uint8Array
  privateKey: Uint8Array
}

export function sign(privateKey: Uint8Array, message: Uint8Array): Uint8Array

export function verify(publicKey: Uint8Array, message: Uint8Array, signature: Uint8Array): boolean
```
