#include <stdlib.h>
#include <stdio.h>

// 64 bit
typedef long long usize;
extern void show_loading_animation(usize, usize, usize (*prog_fn)());

usize progress_reporter() {
    return 80l;
}

int main(void) {
    show_loading_animation(0, 100, &progress_reporter);
    return 0;
}