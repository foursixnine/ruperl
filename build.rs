extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Tell cargo to tell rustc to link the system perl
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));

    // println!("cargo:rerun-if-changed=perl-boilerplate/wrapper.h");
    // generate the perlxsi.c file
    Command::new("perl")
        .args(&["-MExtUtils::Embed", "-e", "xsinit", "--", "-o", "perlxsi.c"])
        .status()
        .expect("Failed to generate perlxsi.c");

    // generate the ccopts (c flags) from perl
    let ccopts = Command::new("perl")
        .args(&["-MExtUtils::Embed", "-e", "ccopts"])
        .output()
        .expect("Failed to get ccopts")
        .stdout;
    let ccopts = String::from_utf8_lossy(&ccopts).trim().to_string();
    let ccopts_iter = ccopts.split_whitespace();

    // Get ldopts from Perl and add them to the linker flags
    let ldopts = Command::new("perl")
        .args(&["-MExtUtils::Embed", "-e", "ldopts"])
        .output()
        .expect("Failed to get ldopts")
        .stdout;
    let ldopts_command = String::from_utf8_lossy(&ldopts);
    let ldopts_iter = ldopts_command.split_whitespace();
    let perl_inc = Command::new("perl")
        .args(&["-MExtUtils::Embed", "-e", "perl_inc"])
        .output()
        .expect("Failed to get perl_inc")
        .stdout;
    let perl_inc_command = String::from_utf8_lossy(&perl_inc).trim().to_string();

    // Tell cargo to link the perlxsi object file
    for opt in ldopts_iter {
        println!("cargo:rustc-link-arg={}", opt);
    }
    println!("cargo:rustc-link-search=native={}", out_dir.display());

    // Compile perlxsi.c with ccopts
    let mut build = cc::Build::new();
    build.file("perlxsi.c");
    for opt in ccopts_iter {
        build.flag(opt);
    }
    build.compile("perlxsi");

    // println!("cargo:rustc-link-lib=static=perlxsi");
    // println!("cargo:rustc-link-search=native={}", out_dir.display());

    // let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    // let perlxsi_a = out_dir.join("libperlxsi.a");

    let bindings = bindgen::Builder::default()
        .header("perl-boilerplate/wrapper.h")
        .clang_arg(perl_inc_command)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
