export interface KeyPair {
  publicKey: Buffer
  privateKey: Buffer
}

export function createKeyPair(privateKey: Buffer): KeyPair
export function generateKeyPair(): KeyPair
export function sign(privateKey: Buffer, message: Buffer): Buffer
export function verify(publicKey: Buffer, message: Buffer, signature: Buffer): boolean
