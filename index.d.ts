/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export interface Ed25519Keypair {
  publicKey: Buffer
  privateKey: Buffer
}
export function generateKeyPair(): Ed25519Keypair
export function createKeyPair(privateKey: Buffer): Ed25519Keypair
export function sign(privateKey: Buffer, message: Buffer): Buffer
export function verify(publicKey: Buffer, message: Buffer, signatureBuffer: Buffer): boolean
