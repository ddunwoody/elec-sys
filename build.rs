/*
 * Copyright © 2023 David Dunwoody.
 *
 * All rights reserved.
 */

fn main() {
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    println!("cargo:rerun-if-env-changed=LIBACFUTILS_REDIST");
    println!("cargo:rerun-if-env-changed=LIBELEC_REDIST");

    let elec_redist_path = std::path::Path::new(env!("LIBELEC_REDIST"));

    configure(&elec_redist_path);

    #[cfg(feature = "generate-bindings")]
    generate_bindings(&elec_redist_path);
}

fn configure(elec_redist_path: &std::path::Path) {
    let dir = match get_target() {
        Target::Windows => "mingw64",
        Target::MacOs => "mac64",
        Target::Linux => "lin64",
    };

    println!(
        "cargo:rustc-link-search={}/{dir}/lib",
        elec_redist_path.display()
    );
    println!("cargo:rustc-link-lib=static=elec");
}

#[cfg(feature = "generate-bindings")]
fn generate_bindings(elec_redist_path: &std::path::Path) {
    let xplane_sdk_path = std::path::Path::new(env!("XPLANE_SDK"));
    let acfutils_redist_path = std::path::Path::new(env!("LIBACFUTILS_REDIST"));
    let header = format!("{}/include/libelec.h", elec_redist_path.display());
    println!("{header}");

    bindgen::Builder::default()
        .header(&header)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(LibElecCallbacks))
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .clang_args([
            format!("-I{}/include", acfutils_redist_path.display()),
            format!("-I{}/CHeaders/XPLM", xplane_sdk_path.display()),
            format!("-D{}", get_xp_def()),
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

#[cfg(feature = "generate-bindings")]
#[derive(Debug)]
struct LibElecCallbacks;

#[cfg(feature = "generate-bindings")]
impl bindgen::callbacks::ParseCallbacks for LibElecCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        if original_item_name.starts_with("libelec_") {
            Some(original_item_name[8..].to_string())
        } else {
            None
        }
    }
}
