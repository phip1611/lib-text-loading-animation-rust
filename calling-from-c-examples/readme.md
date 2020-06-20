# Calling library *text-loading-animation* from C.

## Prerequisites
- this special setup (make, gcc) works only on UNIX-like systems (MacOS, Linux)
- Rust builds a shared object (=dynamically linkable library)

## How it works
- gcc links this shared object into main.c during compile: see Makefile
- in main.c there is a reference to an external function: see main.c
- when main.c is executed the linker needs to know about the shared object: see run.sh.
