extern crate bindgen;
extern crate pkg_config;

fn main() {
    let _ = pkg_config::probe_library("libobs").unwrap();

    let bindings = bindgen::builder()
        .header("wrapper.h")
        .blacklist_type("_bindgen_ty_2")
        .generate()
        .expect("Unable to generate libOBS bindings. Do you have OBS >= 23.0.0 installed?");

    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings in the directory");
}
