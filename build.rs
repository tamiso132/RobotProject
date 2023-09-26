const PATH_TO_C: &str = "bluez/";
use std::ops::Add;

use bindgen::builder;
use glob::glob;
fn main() {
    let include = format!("{}include", PATH_TO_C);

    let entries = glob("bluez/**/*.c").expect("Failed to read glob pattern");

    let bindings = builder()
        .header("bluez/bluegen.h")
        .clang_arg("-Ibluez/include")
        .generate()
        .unwrap();
    bindings.write_to_file("src/bindings.rs").unwrap();

    let mut c_paths = vec![];
    for c in entries {
        if c.as_ref().unwrap().to_string_lossy().contains("main") {
            continue;
        }
        let s = String::new();
        let v = s.add(c.unwrap().to_str().unwrap());
        c_paths.push(v);
    }

    cc::Build::new()
        .files(c_paths.iter())
        .include(include)
        .compile("bro");

    println!("cargo:rerun-if-changed=bluez/"); // Replace 'bluetooth' with the actual library name
    println!("cargo:rustc-link-lib=bluetooth");
}
