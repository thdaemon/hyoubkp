# Hyoubkp Mobile for iOS

## Preparing the Toolchain

I don't have a Mac, so only cross-compiling on Linux is supported! :-)

1. Install [cctools-port](https://github.com/tpoechtrager/cctools-port)
2. Add `/path/to/cctools-port-build-dir/target/bin` to your `PATH`
3. Set the iPhoneOS SDK directory as the `SDKROOT` environment variable
4. (Optional) If you're using a non-default prefix, edit the `toolchain.cmake` and `.cargo/config.toml` files
5. Run `rustup target add aarch64-apple-ios`

## Building the Project

```
cargo build -r -p hyoubkp_mobile_ios --features hyoubkp/tokmap_rule --target aarch64-apple-ios
```

## Creating the App Bundle and IPA

```
cd crates/hyoubkp_mobile_ios
cp -f ../../target/aarch64-apple-ios/release/HyoubkpMobile Payload/HyoubkpMobile.app/
zip -r HyoubkpMobile.ipa Payload/
```

After that, you can sideload the IPA onto your phone. :-)

By the way, if you want to sign your app, you might want to check out [rcodesign](https://github.com/indygreg/apple-platform-rs/tree/main/apple-codesign). And if you want to sideload, you might want to check out [Sideloader](https://github.com/Dadoum/Sideloader).