import { randomBytes } from 'crypto'

import b from 'benny'

import { sign, generateKeyPair as generateKeyPairNapi, verify } from '../index'

const { generateKeyPair, calculateSignature, verifySignature } = require('curve25519-n')

const message = Buffer.from('hello world! 👀')

const NanKeyPair = generateKeyPair(randomBytes(32))
const NapiKeyPair = generateKeyPairNapi()

const NapiSignature = sign(NapiKeyPair.privateKey, message)
const NanSignature = calculateSignature(NanKeyPair.privKey, message)

async function run() {
  await b.suite(
    'generateKeyPair',

    b.add('napi', () => {
      generateKeyPairNapi()
    }),

    b.add('nan', () => {
      generateKeyPair(randomBytes(32))
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'sign',

    b.add('napi', () => {
      sign(NapiKeyPair.privateKey, message)
    }),

    b.add('nan', () => {
      calculateSignature(NanKeyPair.privKey, message)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'sign',

    b.add('napi', () => {
      sign(NapiKeyPair.privateKey, message)
    }),

    b.add('nan', () => {
      calculateSignature(NanKeyPair.privKey, message)
    }),

    b.cycle(),
    b.complete(),
  )

  await b.suite(
    'verify',

    b.add('napi', () => {
      verify(NapiKeyPair.publicKey, message, NapiSignature)
    }),

    b.add('nan', () => {
      verifySignature(NanKeyPair.pubKey, message, NanSignature)
    }),

    b.cycle(),
    b.complete(),
  )
}

run().catch((e) => {
  console.error(e)
})
