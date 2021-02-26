import test from 'ava'

import { generateKeyPair as generateKeyPairNapi, createKeyPair, sign, verify } from '../index'

test('should be able to sign and verify', (t) => {
  const message = Buffer.from('hello world 👀')
  const { publicKey, privateKey } = generateKeyPairNapi()
  const signature = sign(privateKey, message)
  t.true(verify(publicKey, message, signature))
})

test('should be able to create key pair from generated private key', (t) => {
  const { publicKey, privateKey } = generateKeyPairNapi()
  const { publicKey: pubKey } = createKeyPair(privateKey)
  t.deepEqual(publicKey, pubKey)
})
