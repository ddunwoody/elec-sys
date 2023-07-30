#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]

include!("bindings.rs");

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::path::Path;
    use std::time::Duration;

    #[test]
    fn can_load_and_run_system() {
        let filename = Path::new(env!("CARGO_MANIFEST_DIR")).join("resources/test/test.net");
        unsafe {
            acfutils_sys::crc64_init();
            acfutils_sys::crc64_srand(0);
        }

        let filename = CString::new(filename.to_str().unwrap()).unwrap();
        unsafe {
            let sys = crate::libelec_new(filename.as_ptr());
            assert!(!sys.is_null());
            assert!(crate::libelec_sys_can_start(sys));

            let comp_c = CString::new("MAIN_BATT").unwrap();
            let comp = crate::libelec_comp_find(sys, comp_c.as_ptr());

            crate::libelec_sys_start(sys);

            std::thread::sleep(Duration::from_millis(50));

            assert!(!comp.is_null());
            let volts = crate::libelec_comp_get_out_volts(comp);
            assert_eq!(volts, 25.4);

            crate::libelec_sys_stop(sys);
            crate::libelec_destroy(sys);
        }
    }
}
