use ruperl::*;

fn main() {
    println!(
        "Find the stuff at: {}",
        concat!(env!("OUT_DIR"), "/bindings.rs")
    );
    unsafe {
        PL_perl_destruct_level = 1;
        Perl_sys_init();
        let myperl = perl_alloc();
        perl_construct(myperl);
        perl_run(myperl);

        //perl_destruct(myperl);
        //perl_free(myperl);
    }
    println!("We got here!");
}
