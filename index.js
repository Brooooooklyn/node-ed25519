const { loadBinding } = require('@node-rs/helper')

/**
 * __dirname means load native addon from current dir
 * 'ed25519' means native addon name is `ed25519`
 * the first arguments was decided by `napi.name` field in `package.json`
 * the second arguments was decided by `name` field in `package.json`
 * loadBinding helper will load `ed25519.[PLATFORM].node` from `__dirname` first
 * If failed to load addon, it will fallback to load from `@napi-rs/ed25519-[PLATFORM]`
 */
module.exports = loadBinding(__dirname, 'ed25519', '@napi-rs/ed25519')
