use cmake::Config;
use std::env;

fn main() {
    //return;

    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    if env::var("CARGO_CFG_TARGET_VENDOR").unwrap() == "apple" {
        let dst = Config::new("libui")
        .define(
            "CMAKE_TOOLCHAIN_FILE",
            format!("{}/toolchain.cmake", crate_dir).as_str(),
        )
        .build();

        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib=static=hmui_ios");
        println!("cargo:rustc-link-lib=framework=Foundation");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");
        println!("cargo:rustc-link-lib=framework=UIKit");
    }
}
