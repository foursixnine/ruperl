use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-search=/opt/homebrew/opt/perl/lib/perl5/5.38/darwin-thread-multi-2level/CORE");
    println!("cargo:rustc-link-lib=perl");

    let bindings = bindgen::Builder::default()
        .header("perl-boilerplate/wrapper.h")
        .clang_arg("-I/opt/homebrew/opt/perl/lib/perl5/5.38/darwin-thread-multi-2level/CORE")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
