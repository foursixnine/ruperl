// SPDX-License-Identifier: Apache-2.0
// Author: Santiago Zarate <github@zarate.co>
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use ruperl::*;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

// as this is more of an example, this should be moved to
// examples/ directory with its own documentation and cargo.toml

// TODO: boot_DynaLoader should be moved to lib.rs
extern "C" {
    fn boot_DynaLoader(perl_interpreter: *mut PerlInterpreter, cv: *mut CV);
}

// lets hope copilot's code is correct
// TODO: move this to lib.rs
unsafe extern "C" fn xs_init_rust(perl_interpreter: *mut PerlInterpreter) {
    let file = std::ffi::CString::new(file!()).expect("CString::new failed");

    let perl_sub = CString::new("DynaLoader::boot_DynaLoader").unwrap();

    println!("file to load: {:?}", file);
    Perl_newXS(
        perl_interpreter,
        perl_sub.as_ptr() as *const c_char,
        Some(boot_DynaLoader),
        file.as_ptr(),
    );
}

fn main() {
    println!(
        "Find the stuff at: {}",
        concat!(env!("OUT_DIR"), "/bindings.rs")
    );
    //let argv: Vec<String> = env::args().collect();
    let args: Vec<String> = env::args().collect();

    //let embedding = vec!["-e", "showtime.pl", ""];
    let embedding = args.clone();

    let mut perl_parse_args: Vec<*mut c_char> = embedding
        .iter()
        .map(|arg| arg.as_ptr() as *mut c_char) // Convert each CString to *mut c_char
        .collect();
    perl_parse_args.push(ptr::null_mut()); // Add a null pointer to the end of the list

    let flags = G_DISCARD | G_NOARGS;
    let flags_ptr: i32 = flags as i32;
    let perl_sub = perl_parse_args[2]; // use clap crate instead for args

    // I should use a match block to know whether to continue or not
    // if perl_sub != "showtime"  || perl_sub != "get_quick_headers" {
    //     println!("No sub to call");
    //     return;
    // }

    // for arg in &perl_parse_args {
    //     println!("arg: {:?}", unsafe { CString::from_raw(*arg) });
    // }

    unsafe {
        Perl_sys_init3(
            &mut perl_parse_args.len().try_into().unwrap() as *mut i32,
            &mut perl_parse_args.as_mut_ptr(),
            ptr::null_mut(),
        );

        let myperl = perl_alloc();
        (*myperl).Iperl_destruct_level = 1;

        perl_construct(myperl);
        //perl_run(myperl);
        // there has to be a nicer way to do this
        let _xs_init: Option<unsafe extern "C" fn(*mut interpreter)> = Some(xs_init_rust);
        perl_parse(
            myperl,
            _xs_init,
            perl_parse_args.len().try_into().unwrap(),
            perl_parse_args.as_mut_ptr(),
            ptr::null_mut(),
        );

        Perl_call_argv(myperl, perl_sub, flags_ptr, perl_parse_args.as_mut_ptr());

        //Perl_call_argv(myperl,"print hello", FILE_FLAGS, perl_parse_args.as_ref());

        perl_destruct(myperl);
        perl_free(myperl);
        Perl_sys_term();
    }

    println!("Finished rust's execution.");
}
