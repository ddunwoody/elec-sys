/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */

fn main() {
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    println!("cargo:rerun-if-env-changed=LIBELEC");

    let elec_path = std::path::Path::new(env!("LIBELEC"));

    configure(elec_path);

    #[cfg(feature = "generate-bindings")]
    generate_bindings(elec_path);
}

fn configure(elec_path: &std::path::Path) {
    let dir = match get_target() {
        Target::Windows => "mingw64",
        Target::MacOs => "mac64",
        Target::Linux => "lin64",
    };

    println!("cargo:rustc-link-search={}/{dir}/lib", elec_path.display());
    println!("cargo:rustc-link-lib=static=elec");
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(elec_path: &std::path::Path) {
    let xplane_sdk_path = std::path::Path::new(env!("XPLANE_SDK"));
    let acfutils_path = std::path::Path::new(env!("LIBACFUTILS"));
    let header = format!("{}/include/libelec.h", elec_path.display());
    println!("cargo:rerun-if-changed={header}");

    bindgen::Builder::default()
        .header(&header)
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args([
            "-std=c99",
            &format!("-I{}/libacfutils-redist/include", acfutils_path.display()),
            &format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()),
            &format!("-D{}", get_xp_def()),
        ])
        .allowlist_file(header)
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings");
}

enum Target {
    Windows,
    MacOs,
    Linux,
}

fn get_target() -> Target {
    let target = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target == "macos" {
        Target::MacOs
    } else if target == "windows" {
        Target::Windows
    } else if target == "linux" {
        Target::Linux
    } else {
        panic!("Unsupported target: {target}");
    }
}

#[cfg(feature = "generate-bindings")]
fn get_xp_def() -> &'static str {
    match get_target() {
        Target::Windows => "IBM",
        Target::MacOs => "APL",
        Target::Linux => "LIN",
    }
}
