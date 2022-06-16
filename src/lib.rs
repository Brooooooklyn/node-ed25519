#![deny(clippy::all)]

use ed25519::signature::{Signer, Verifier};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use napi::bindgen_prelude::*;
use napi_derive::napi;
use rand::rngs::OsRng;

#[cfg(all(any(windows, unix), not(target_env = "musl"), not(debug_assertions)))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi(object)]
pub struct Ed25519Keypair {
  pub public_key: Buffer,
  pub private_key: Buffer,
}

#[napi]
pub fn generate_key_pair() -> Result<Ed25519Keypair> {
  let mut csprng = OsRng {};

  let keypair = Keypair::generate(&mut csprng);
  let secret: Buffer = keypair.secret.as_ref().to_owned().into();
  let public: Buffer = keypair.public.as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public,
    private_key: secret,
  })
}

#[napi]
pub fn create_key_pair(private_key: Buffer) -> Result<Ed25519Keypair> {
  let secret_key = SecretKey::from_bytes(&private_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid privateKey {}", e)))?;
  let public_key: PublicKey = (&secret_key).into();
  let secret: Buffer = secret_key.as_ref().to_owned().into();
  let public: Buffer = public_key.as_bytes().to_vec().into();
  Ok(Ed25519Keypair {
    public_key: public,
    private_key: secret,
  })
}

#[napi]
pub fn sign(private_key: Buffer, message: Buffer) -> Result<Buffer> {
  let secret_key = SecretKey::from_bytes(&private_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid privateKey {}", e)))?;
  let public_key: PublicKey = (&secret_key).into();
  let key_pair = Keypair {
    secret: secret_key,
    public: public_key,
  };
  let signature: Signature = key_pair.sign(&message);
  Ok(signature.to_bytes().to_vec().into())
}

#[napi]
pub fn verify(public_key: Buffer, message: Buffer, signature_buffer: Buffer) -> Result<bool> {
  let signature = Signature::from_bytes(&signature_buffer)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid signature {}", e)))?;
  let pub_key = PublicKey::from_bytes(public_key.as_ref())
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key {}", e)))?;

  Ok(pub_key.verify(&message, &signature).is_ok())
}
