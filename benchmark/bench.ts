import { Bench } from 'tinybench'
import { utils, getPublicKey, verify as verifyJs, sign as signJs, etc } from '@noble/ed25519'
import { sha512 } from '@noble/hashes/sha512'

import { sign, generateKeyPair as generateKeyPairNapi, verify } from '../index.js'

etc.sha512Sync = (...m) => sha512(etc.concatBytes(...m))

const message = Buffer.from('hello world! 👀')

const JsPrivKey = utils.randomPrivateKey()
const JsPubKey = getPublicKey(JsPrivKey)
const NapiKeyPair = generateKeyPairNapi()

const NapiSignature = sign(NapiKeyPair.privateKey, message)
const JsSignature = signJs(message, JsPrivKey)

const b = new Bench()

b.add('@napi-rs/ed25519', () => {
  generateKeyPairNapi()
})

b.add('@noble/ed25519', () => {
  const priv = utils.randomPrivateKey()
  getPublicKey(priv)
})

await b.run()

console.info('generateKeyPair')
console.table(b.table())

b.add('@napi-rs/ed25519', () => {
  sign(NapiKeyPair.privateKey, message)
})

b.add('@noble/ed25519', () => {
  signJs(message, JsPrivKey)
})

await b.run()

console.info('sign')
console.table(b.table())

b.add('@napi-rs/ed25519', () => {
  verify(NapiKeyPair.publicKey, message, NapiSignature)
})

b.add('@noble/ed25519', () => {
  verifyJs(JsSignature, message, JsPubKey)
})

await b.run()

console.info('verify')
console.table(b.table())
