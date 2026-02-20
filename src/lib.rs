#![deny(clippy::all)]

use ed25519_dalek::{
  Signature, Signer, SigningKey, VerifyingKey, PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH,
};
use getrandom::{rand_core::UnwrapErr, SysRng};
use napi::bindgen_prelude::*;
use napi_derive::napi;

#[napi(object)]
pub struct Ed25519Keypair {
  pub public_key: Uint8Array,
  pub private_key: Uint8Array,
}

#[napi]
pub fn generate_key_pair() -> Result<Ed25519Keypair> {
  let keypair = SigningKey::generate(&mut UnwrapErr(SysRng));
  let secret = keypair.as_bytes().to_vec().into();
  let public = keypair.verifying_key().as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public,
    private_key: secret,
  })
}

#[napi]
pub fn create_key_pair(private_key: &[u8]) -> Result<Ed25519Keypair> {
  if private_key.len() != SECRET_KEY_LENGTH {
    return Err(Error::new(
      Status::InvalidArg,
      "Invalid privateKey length, must be 32 bytes".to_string(),
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(private_key);
  let secret_key = SigningKey::from_bytes(&pk);
  let public_key = secret_key.verifying_key();
  let secret = secret_key.as_bytes().to_vec().into();
  let public = public_key.as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public,
    private_key: secret,
  })
}

#[napi]
pub fn sign(private_key: &[u8], message: &[u8]) -> Result<Uint8Array> {
  if private_key.len() != SECRET_KEY_LENGTH {
    return Err(Error::new(
      Status::InvalidArg,
      "Invalid privateKey length, must be 32 bytes".to_string(),
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(private_key);
  let key_pair = SigningKey::from_bytes(&pk);
  let signature = key_pair.sign(message);
  Ok(signature.to_bytes().to_vec().into())
}

#[napi]
pub fn verify(public_key: &[u8], message: &[u8], signature_buffer: &[u8]) -> Result<bool> {
  let signature = Signature::from_slice(signature_buffer)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid signature {e}")))?;
  if public_key.len() != PUBLIC_KEY_LENGTH {
    return Err(Error::new(
      Status::InvalidArg,
      "Invalid privateKey length, must be 32 bytes".to_string(),
    ));
  }
  let mut pk = [0; 32];
  pk.copy_from_slice(public_key);
  let pub_key = VerifyingKey::from_bytes(&pk)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key {e}")))?;

  Ok(pub_key.verify_strict(message, &signature).is_ok())
}
