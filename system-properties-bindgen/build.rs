use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=external/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("bindgen/system_properties.h")
        .allowlist_function("__system_property_find")
        .allowlist_function("__system_property_foreach")
        .allowlist_function("__system_property_read_callback")
        .allowlist_function("__system_property_set")
        .allowlist_function("__system_property_wait")
        .blocklist_type("timespec")
        .raw_line("use libc::timespec;")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Failed to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Failed to write bindings");
}
