// SPDX-License-Identifier: Apache-2.0
// Author: Santiago Zarate <github@zarate.co>
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use ruperl::*;
use std::env;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::thread::sleep;

// // TODO: boot_DynaLoader should be moved to lib.rs
// extern "C" {
//     fn boot_DynaLoader(perl_interpreter: *mut PerlInterpreter, cv: *mut CV);
// }

// // lets hope copilot's code is correct
// // TODO: move this to lib.rs
// pub unsafe extern "C" fn xs_init_rust(perl_interpreter: *mut PerlInterpreter) {
//     // pub unsafe extern "C" fn xs_init_rust(perl_interpreter: *mut PerlInterpreter, args: Vec<String>) {
//     // println!("{:?}", args); // debug print the args
//     let file = std::ffi::CString::new(file!()).expect("CString::new failed");

//     let perl_sub = CString::new("DynaLoader::boot_DynaLoader").unwrap();

//     println!("xs_init_rust in example file to load: {:?}", file);
//     Perl_newXS(
//         perl_interpreter,                   // interpreter,
//         perl_sub.as_ptr() as *const c_char, // name,
//         Some(boot_DynaLoader),              // subaddr,
//         file.as_ptr(),                      // filename,
//     );
//     println!("xs_init_rust loaded: {:?}", file);
//     dbg!("xs_init_rust loaded: {:?}", file);
//     println!("Leaving xs_init_rust in example file.");
// }

fn main() {
    println!(
        "Find the stuff at: {}",
        concat!(env!("OUT_DIR"), "/bindings.rs")
    );
    //let argv: Vec<String> = env::args().collect();
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <perl_script> <perl_sub>", args[0]);
        return;
    }
    //
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

    println!("Perl sub: {:?}", unsafe { CString::from_raw(perl_sub) });

    // I should use a match block to know whether to continue or not
    // if perl_sub != "showtime"  || perl_sub != "get_quick_headers" {
    //     println!("No sub to call");
    //     return;
    // }

    for arg in args.clone() {
        dbg!("debug: arg: {:?}", arg);
    }

    let mut perl_parse_env_args =
        vec!["PERL5LIB=$PERL5LIB:/Users/foursixnine/perl5/lib/perl5:examples/call_argv/lib:examples/call_argv"];
    let perl_exit_flags = PERL_EXIT_DESTRUCT_END;

    let myperl: *mut PerlInterpreter;

    unsafe {
        Perl_sys_init(
            &mut perl_parse_args.len().try_into().unwrap(),
            &mut perl_parse_args.as_mut_ptr(),
        );

        println!("Perl initialized");
        myperl = perl_alloc();
        (*myperl).Iperl_destruct_level = 1;
        (*myperl).Iorigalen = 1;
        (*myperl).Iexit_flags = perl_exit_flags.try_into().unwrap();

        perl_construct(myperl);
    }
    println!("Call perl parse, sleeping 10 seconds before that");
    // sleep until n seconds have passed
    let mut seconds = 10;
    while seconds > 0 {
        println!("Sleeping 1 second, {} ahead", seconds);
        sleep(std::time::Duration::from_secs(1));
        seconds -= 1;
    }
    unsafe {
        //perl_run(myperl);
        // there has to be a nicer way to do this
        //let _xs_init: Option<unsafe extern "C" fn(*mut interpreter)> = Some(ruperl::xs_init_rust);
        perl_parse(
            myperl,
            Some(ruperl::xs_init_rust),
            perl_parse_args.len().try_into().unwrap(),
            perl_parse_args.as_mut_ptr(),
            perl_parse_env_args.as_mut_ptr() as *mut *mut c_char,
        );

        println!("Call call_argv, sleeping 10 seconds before that");
        // sleep until n seconds have passed
        let mut seconds = 10;
        while seconds > 0 {
            println!("Sleeping 1 second, {} ahead", seconds);
            sleep(std::time::Duration::from_secs(1));
            seconds -= 1;
        }

        Perl_call_argv(myperl, perl_sub, flags_ptr, perl_parse_args.as_mut_ptr());

        perl_destruct(myperl);
        perl_free(myperl);
        Perl_sys_term();
    }

    println!("Finished rust's execution.");
}
