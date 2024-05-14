#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::ffi::CString;
use std::os::raw::c_char;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// TODO: boot_DynaLoader should be moved to lib.rs
extern "C" {
    fn boot_DynaLoader(perl_interpreter: *mut PerlInterpreter, cv: *mut CV);
}

// lets hope copilot's code is correct
// TODO: move this to lib.rs
pub unsafe extern "C" fn xs_init_rust(perl_interpreter: *mut PerlInterpreter) {
    // pub unsafe extern "C" fn xs_init_rust(perl_interpreter: *mut PerlInterpreter, args: Vec<String>) {
    // println!("{:?}", args); // debug print the args
    let file = std::ffi::CString::new(file!()).expect("CString::new failed");

    let perl_sub = CString::new("DynaLoader::boot_DynaLoader").unwrap();

    println!("xs_init_rust in example file to load: {:?}", file);
    Perl_newXS(
        perl_interpreter,                   // interpreter,
        perl_sub.as_ptr() as *const c_char, // name,
        Some(boot_DynaLoader),              // subaddr,
        file.as_ptr(),                      // filename,
    );
    println!("xs_init_rust loaded: {:?}", file);
    dbg!("xs_init_rust loaded: {:?}", file);
    println!("Leaving xs_init_rust in example file.");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    use std::ptr::null_mut;

    #[test]
    fn test_construct() {
        let perl_exit_flags = PERL_EXIT_DESTRUCT_END;
        let myperl: *mut PerlInterpreter;
        
        unsafe {

            myperl = perl_alloc();
            (*myperl).Iperl_destruct_level = 1;
            (*myperl).Iorigalen = 1;
            (*myperl).Iexit_flags = perl_exit_flags.try_into().unwrap();
    
            perl_construct(myperl);

            perl_run(myperl);
            perl_destruct(myperl);
            perl_free(myperl);
        }
        debug_assert!(myperl.is_null());
    }

    #[test]
    fn test_parse() {
        let perl_exit_flags = PERL_EXIT_DESTRUCT_END;
        let myperl: *mut PerlInterpreter;
        
        unsafe {

            myperl = perl_alloc();
            (*myperl).Iperl_destruct_level = 1;
            (*myperl).Iorigalen = 1;
            (*myperl).Iexit_flags = perl_exit_flags.try_into().unwrap();
    
            perl_construct(myperl);

            // perl_run(myperl);
            perl_parse(
                myperl,
                Some(xs_init_rust),
                0,
                null_mut(),
                null_mut(),
            );
            perl_destruct(myperl);
            perl_free(myperl);
        }
        debug_assert!(myperl.is_null());
    }
}
