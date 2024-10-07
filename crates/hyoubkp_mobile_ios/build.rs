use cmake::Config;
use std::{env, process::Command};

fn main() {
    let git_hash = match Command::new("git").args(&["rev-parse", "--short", "HEAD"]).output() {
        Ok(output) => String::from_utf8(output.stdout).unwrap(),
        Err(_) => String::from("unknown"),
    };
    println!("cargo:rustc-env=HM_GIT_HASH={}", git_hash);
    println!("cargo:rerun-if-changed=../../.git/HEAD");

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
        println!("cargo:rustc-link-lib=framework=WebKit");

        println!("cargo:rerun-if-changed=./libui");
    }
}
