export function generateKeyPair(): {
  publicKey: Buffer
  privateKey: Buffer
}
export function sign(privateKey: Buffer, message: Buffer): Buffer
export function verify(publicKey: Buffer, message: Buffer, signature: Buffer): boolean
