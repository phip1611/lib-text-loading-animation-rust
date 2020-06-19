#include <stdlib.h>
#include <stdio.h>

typedef unsigned long long usize; // 64 bit
extern void show_loading_animation_ffi(usize, usize, usize (*prog_fn)());

usize progress_reporter() {
    return (usize) 80;
}

int main(void) {
    show_loading_animation_ffi(0, 100, progress_reporter);
    return 0;
}