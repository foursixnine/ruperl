// following https://rust-lang.github.io/rust-bindgen/tutorial-3.html
#include <perl.h>
#include <EXTERN.h>
#include "XSUB.h"

/* 1 = clean out filename's symbol table after each request,
   0 = don't
*/
#ifndef DO_CLEAN
#define DO_CLEAN 0
#endif

#define BUFFER_SIZE 1024
