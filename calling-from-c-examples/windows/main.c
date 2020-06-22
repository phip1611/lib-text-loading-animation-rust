#include <stdio.h>
#include <Windows.h>

// The target stream.
enum target { TARGET_STDOUT, TARGET_STDERR };
typedef unsigned long long usize_t; // 64 bit / usize on 64 bit machine
extern void show_loading_animation_ffi(usize_t, usize_t, int, usize_t (*prog_fn)());

// shared global var
usize_t counter = 0;

// function that tells the lib how many percent progress we made so far
usize_t progress_reporter() { return counter; }

DWORD WINAPI progress_maker(void) {
    while (counter < 100) {
        counter ++;
        Sleep(100); // Sleep 100 milli seconds
    }
    // When this function returns, the thread goes away. See MSDN for more details.
    return 0;
}


int main(void) {
    CreateThread(NULL, 0, (LPTHREAD_START_ROUTINE) progress_maker, NULL, 0, NULL);
    show_loading_animation_ffi(0, 100, TARGET_STDERR, progress_reporter);
    Sleep(200); // Sleep 100 milli seconds
    // Overwrite last written line from loading animation
    printf("\rdone :)                            \n");
    return 0;
}
