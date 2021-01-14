import { randomBytes } from 'crypto'

import test from 'ava'

import { generateKeyPair as generateKeyPairNapi, createKeyPair as createKeyPairNapi, sign, verify } from '../index'

const { createKeyPair } = require('curve25519-n')

test('should be able to sign and verify', (t) => {
  const message = Buffer.from('hello world 👀')
  const { publicKey, privateKey } = generateKeyPairNapi()
  const signature = sign(privateKey, message)
  t.true(verify(publicKey, message, signature))
})

test('create the same keyPair with curve25519-n', (t) => {
  const rng = randomBytes(32)
  const { privKey } = createKeyPair(rng)
  const { privateKey } = createKeyPairNapi(rng)
  t.deepEqual(privKey, privateKey)
})
