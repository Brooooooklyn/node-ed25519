#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{
  CallContext, ContextlessResult, Env, Error, JsBoolean, JsBuffer, JsObject, Result, Status,
};
use ring::{
  rand,
  signature::{self, KeyPair},
};

#[cfg(all(unix, not(target_env = "musl"), not(target_arch = "aarch64")))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[cfg(all(windows, target_arch = "x86_64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("generateKeyPair", generate_key_pair)?;
  exports.create_named_method("createKeyPair", create_key_pair)?;
  exports.create_named_method("sign", sign)?;
  exports.create_named_method("verify", verify)?;
  Ok(())
}

#[contextless_function]
fn generate_key_pair(env: Env) -> ContextlessResult<JsObject> {
  let rng = rand::SystemRandom::new();
  let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
    .map_err(|e| Error::new(Status::InvalidArg, format!("{}", e)))?;
  let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
    .map_err(|e| Error::new(Status::InvalidArg, format!("{}", e)))?;
  let mut result = env.create_object()?;
  let secret = env
    .create_buffer_with_data(pkcs8_bytes.as_ref().to_owned())?
    .into_raw();
  let public = env
    .create_buffer_with_data(key_pair.public_key().as_ref().to_vec())?
    .into_raw();
  result.set_named_property("privateKey", secret)?;
  result.set_named_property("publicKey", public)?;
  Ok(Some(result))
}

#[js_function(1)]
fn create_key_pair(ctx: CallContext) -> Result<JsObject> {
  let private_key = ctx.get::<JsBuffer>(0)?.into_value()?;

  let mut result = ctx.env.create_object()?;
  let key_pair = signature::Ed25519KeyPair::from_pkcs8(&private_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("Invalid privateKey {}", e)))?;
  let secret = ctx
    .env
    .create_buffer_with_data(private_key.as_ref().to_owned())?
    .into_raw();
  let public = ctx
    .env
    .create_buffer_with_data(key_pair.public_key().as_ref().to_vec())?
    .into_raw();
  result.set_named_property("privateKey", secret)?;
  result.set_named_property("publicKey", public)?;
  Ok(result)
}

#[js_function(2)]
fn sign(ctx: CallContext) -> Result<JsBuffer> {
  let private_key = ctx.get::<JsBuffer>(0)?.into_value()?;
  let message = ctx.get::<JsBuffer>(1)?.into_value()?;

  let key_pair = signature::Ed25519KeyPair::from_pkcs8(&private_key)
    .map_err(|e| Error::new(Status::InvalidArg, format!("{}", e)))?;
  let signature = key_pair.sign(&message);
  ctx
    .env
    .create_buffer_with_data(signature.as_ref().to_vec())
    .map(|b| b.into_raw())
}

#[js_function(3)]
fn verify(ctx: CallContext) -> Result<JsBoolean> {
  let public_key = ctx.get::<JsBuffer>(0)?.into_value()?;
  let message = ctx.get::<JsBuffer>(1)?.into_value()?;
  let signature_buffer = ctx.get::<JsBuffer>(2)?.into_value()?;
  let peer_public_key = signature::UnparsedPublicKey::new(&signature::ED25519, public_key);

  ctx
    .env
    .get_boolean(peer_public_key.verify(&message, &signature_buffer).is_ok())
}
