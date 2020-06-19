#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h> // usleep

/**
 * The target stream.
 */
enum target { TARGET_STDOUT, TARGET_STDERR };
typedef unsigned long long usize_t; // 64 bit / usize on 64 bit machine
extern void show_loading_animation_ffi(usize_t, usize_t, int, usize_t (*prog_fn)());

// shared global var
usize_t counter;

// function that tells the lib how many percent progress we made so far
usize_t progress_reporter() { return counter; }

// function where our worker thread makes progress
void * progress_maker() {
    while(counter < 100) {
        counter++;
        usleep(1000 * 50);
    }
    pthread_exit(NULL);
}

int main(void) {
    pthread_t t1;
    // start thread
    pthread_create(&t1, NULL, progress_maker, NULL);
    show_loading_animation_ffi(0, 100, TARGET_STDERR, progress_reporter);
    // Overwrite last written line from loading animation
    printf("\rdone :)                            \n");
    return 0;
}