# Hyoubkp mobile for iOS

## Prepare the toolchain

I haven't a Mac, so only cross-compile on Linux is supported! :-)

1. Install the [cctools-port](https://github.com/tpoechtrager/cctools-port)
2. Add the `/path/to/cctools-port-build-dir/target/bin` to `PATH`
3. Add the iPhoneOS SDK directory to env-var `SDKROOT`
4. Edit the `toolchain.cmake` file, change toolchain binaries directory
5. `rustup target add aarch64-apple-ios`

## Build

```
cargo build -r -p hyoubkp_mobile_ios --features hyoubkp/tokmap_user --target aarch64-apple-ios
```

## Make the app bundle and IPA

```
cd crates/hyoubkp_mobile_ios
cp -f ../../target/aarch64-apple-ios/release/HyoubkpMobile Payload/HyoubkpMobile.app/
zip -r HyoubkpMobile.ipa Payload/
```

Then, you can sideload the ipa to your phones. :-)
