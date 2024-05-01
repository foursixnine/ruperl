# RuPerl - Rust with embedded Perl

This is something that shouldn't exist; but it does. It's a Perl interpreter embedded in Rust.

Once cloned, you can run the following commands to see it in action:

- `cargo run --verbose -- hello.pm showtime`
- `cargo run --verbose -- hello.pm get_quick_headers`

## How it works

There is a lot of autogenerated code, mainly for two things:

- `bindings.rs` and `wrapper.h`; I made a lot of assumptions and `perlxsi.c` may or may not be necessary in the future (see `main::xs_init_rust`), depends on how bad or terrible my `C` knowledge is by the time you're reading this.
- `xs_init_rust` function is the one that does the magic, as far as my understanding goes, by hooking up `boot_DynaLoader` to [DynaLoader](https://metacpan.org/pod/DynaLoader) in Perl via ffi.

With those two bits in place, and thanks to the magic of the `bindgen` crate, and after some initialization, I decided to use `Perl_call_argv`, do note that `Perl_` in this case comes from bindgen, I might change later the convention to `ruperl` or something to avoid confusion between that a and `perl_parse` or `perl_alloc` which (if I understand correctly) are exposed directly by the ffi interface.

What I ended up doing, is passing the same list of arguments (for now, or at least for this PoC), directly to `Perl_call_argv`, which will in turn, take the third argument and pass it verbatim as the [call_argv](https://perldoc.perl.org/perlcall#Using-call_argv) 

```
        Perl_call_argv(myperl, perl_sub, flags_ptr, perl_parse_args.as_mut_ptr());
```

Right now hello.pm defines two sub routines, one to open a file, write something and print the time to stdout, and a second one that will query my blog, and show the headers. This is only example code, but enough
to demostrate that the DynaLoader works, and that the embedding also works :)

![itsalive](https://media.giphy.com/media/v1.Y2lkPTc5MGI3NjExd3pkbzJweGl3aG8waG0xeHJpZDU5bXQ4bnRqdjlmdGNjdXA1eXRteCZlcD12MV9pbnRlcm5hbF9naWZfYnlfaWQmY3Q9Zw/tze1mGedykiuk/giphy.gif)

I got most of this working by following the [perlembed](https://perldoc.perl.org/perlembed) guide.

## Why?

> Why not?.

I want to see if I can embed also python in the same binary, so I can call native perl, from native python and see how I can fiddle all that into [os-autoinst](https://github.com/os-autoinst/os-autoinst)
