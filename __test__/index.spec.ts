import test from 'ava'

import { generateKeyPair, sign, verify } from '../index'

test('sync function from native code', (t) => {
  const message = Buffer.from('hello world 👀')
  const { publicKey, privateKey } = generateKeyPair()
  const signature = sign(privateKey, message)
  t.true(verify(publicKey, message, signature))
})
