/*
 * Copyright (c) 2023 David Dunwoody.
 *
 * All rights reserved.
 */
#![deny(clippy::all)]
#![warn(clippy::pedantic)]

use build_support::{get_acfutils_cflags, get_target_platform, Platform};
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-env-changed=LIBELEC");
    println!("cargo:rerun-if-env-changed=XPLANE_SDK");
    println!("cargo:rerun-if-env-changed=LIBACFUTILS");

    let acfutils_path = Path::new(env!("LIBACFUTILS"));
    let xplane_sdk_path = Path::new(env!("XPLANE_SDK"));
    let elec_path = Path::new(env!("LIBELEC"));

    let platform = get_target_platform();

    generate_bindings(platform, acfutils_path, xplane_sdk_path, elec_path);

    build(platform, acfutils_path, xplane_sdk_path, elec_path);
}

fn generate_bindings(
    platform: Platform,
    acfutils_path: &Path,
    xplane_sdk_path: &Path,
    elec_path: &Path,
) {
    let mut builder = bindgen::Builder::default()
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .clang_args(build_support::get_acfutils_cflags(
            platform,
            acfutils_path,
            xplane_sdk_path,
        ))
        .blocklist_function("libelec_tie_get_v")
        .blocklist_function("libelec_tie_set_v")
        .blocklist_type("__builtin_va_list")
        .blocklist_type("va_list");

    let cairo_h = format!(
        "{}/{}/include/cairo.h",
        acfutils_path.display(),
        platform.short()
    );
    builder = builder.header(&cairo_h).allowlist_file(&cairo_h);

    for header in get_files(elec_path, "h") {
        println!("cargo:rerun-if-changed={header}");
        builder = builder.header(&header).allowlist_file(&header);
    }

    let bindings =
        PathBuf::from(env::var("OUT_DIR").expect("Could not get OUT_DIR")).join("bindings.rs");

    builder
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(bindings)
        .expect("Couldn't write bindings");
}

fn get_files(elec_path: &Path, ext: &str) -> Vec<String> {
    let path = elec_path.display();
    #[allow(unused_mut)]
    let mut files = vec![
        format!("{path}/src/libelec.{ext}"),
        format!("{path}/src/libelec_drawing.{ext}"),
    ];

    #[cfg(feature = "xplane")]
    files.push(format!("{path}/src/libelec_vis.{ext}"));

    files
}

fn build(platform: Platform, acfutils_path: &Path, xplane_sdk_path: &Path, elec_path: &Path) {
    let mut builder = cc::Build::new();
    for flag in get_acfutils_cflags(platform, acfutils_path, xplane_sdk_path) {
        builder.flag(&flag);
    }
    builder.files(get_files(elec_path, "c"));

    #[cfg(all(feature = "xplane", feature = "datarefs"))]
    builder.define("LIBELEC_WITH_DRS", "1");

    #[cfg(feature = "xplane")]
    builder.define("XPLANE", "1");
    #[cfg(not(feature = "xplane"))]
    builder.define("_LACF_WITHOUT_XPLM", None);

    builder.compile("elec");
}
