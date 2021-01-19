import test from 'ava'

import { generateKeyPair as generateKeyPairNapi, sign, verify } from '../index'

test('should be able to sign and verify', (t) => {
  const message = Buffer.from('hello world 👀')
  const { publicKey, privateKey } = generateKeyPairNapi()
  const signature = sign(privateKey, message)
  t.true(verify(publicKey, message, signature))
})
