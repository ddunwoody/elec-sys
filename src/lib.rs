#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

#[cfg(target_os = "macos")]
include!("bindings.rs");
#[cfg(not(target_os = "macos"))]
include!("bindings-lin.rs");
