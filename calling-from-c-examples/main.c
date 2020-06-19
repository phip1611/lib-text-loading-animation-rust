#include <stdlib.h>
#include <stdio.h>
#include <pthread.h>
#include <unistd.h> // usleep

typedef unsigned long long usize; // 64 bit
extern void show_loading_animation_ffi(usize, usize, usize (*prog_fn)());

// shared global var
usize counter;

// function that tells the lib how many percent progress we made so far
usize progress_reporter() { return counter; }

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
    show_loading_animation_ffi(0, 100, progress_reporter);
    return 0;
}