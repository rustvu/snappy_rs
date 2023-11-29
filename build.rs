//! Build script for snappy_rs

// build.rs receives information from cargo via environment variables.
use std::env;

fn main() {
    let out = cmake::Config::new("snappy").build();
    let build = out.join("build");

    let target_os = env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS is set by cargo.");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").expect("CARGO_CFG_TARGET_ENV is set by cargo.");

    println!("cargo:rustc-link-search=native={}", build.display());
    println!("cargo:rustc-link-lib=static=snappy");

    if target_os.contains("macos") {
        println!("cargo:rustc-link-lib=c++");
    } else if !target_env.contains("msvc") {
        println!("cargo:rustc-link-lib=stdc++");
    }
}