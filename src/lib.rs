#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use ed25519::signature::{Signature as SignatureTrait, Signer, Verifier};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use napi::{
  CallContext, ContextlessResult, Env, Error, JsBoolean, JsBuffer, JsObject, Result, Status,
};
use rand::rngs::OsRng;

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("generateKeyPair", generate_key_pair)?;
  exports.create_named_method("sign", sign)?;
  exports.create_named_method("verify", verify)?;
  Ok(())
}

#[contextless_function]
fn generate_key_pair(env: Env) -> ContextlessResult<JsObject> {
  let mut csprng = OsRng {};

  let mut result = env.create_object()?;
  let keypair = Keypair::generate(&mut csprng);
  let secret = env
    .create_buffer_with_data(keypair.secret.as_ref().to_owned())?
    .into_raw();
  let public = env
    .create_buffer_with_data(keypair.public.as_ref().to_owned())?
    .into_raw();
  result.set_named_property("privateKey", secret)?;
  result.set_named_property("publicKey", public)?;
  Ok(Some(result))
}

#[js_function(2)]
fn sign(ctx: CallContext) -> Result<JsBuffer> {
  let private_key = ctx.get::<JsBuffer>(0)?.into_value()?;
  let message = ctx.get::<JsBuffer>(1)?.into_value()?;

  let secret_key = SecretKey::from_bytes(&private_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid privateKey {}", e)))?;
  let public_key: PublicKey = (&secret_key).into();
  let key_pair = Keypair {
    secret: secret_key,
    public: public_key,
  };
  let signature: Signature = key_pair.sign(&message);
  ctx
    .env
    .create_buffer_with_data(signature.to_bytes().to_vec())
    .map(|b| b.into_raw())
}

#[js_function(3)]
fn verify(ctx: CallContext) -> Result<JsBoolean> {
  let public_key = ctx.get::<JsBuffer>(0)?.into_value()?;
  let message = ctx.get::<JsBuffer>(1)?.into_value()?;
  let signature_buffer = ctx.get::<JsBuffer>(2)?.into_value()?;

  let signature = Signature::from_bytes(&signature_buffer)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid signature {}", e)))?;
  let pub_key = PublicKey::from_bytes(&public_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid public key {}", e)))?;

  ctx
    .env
    .get_boolean(pub_key.verify(&message, &signature).is_ok())
}
