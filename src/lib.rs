/// Contains a function that helps you to display a simple loading animation on your screen.

use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write, stderr};

/// Animation steps per seconds / character changes per second.
const ANIMATION_STEPS_PER_SECOND: u64 = 4;
/// Timeout after the next animation step is printed
const ANIMATION_SLEEP_TIMEOUT: u64 = 1000 / ANIMATION_STEPS_PER_SECOND;

/// The chars for the animation (the loading spinner).
static ANIMATION_STATES: [char; 4] = ['—', '\\', '|', '/'];

/// The destination stream where the loading animation should be written to.
#[derive(Debug)]
pub enum Destination {
    STDOUT,
    STDERR,
}

/// Displays a loading animation on stdout. From and To are both values that are not necessarily
/// percent. They describe to problem range, e.g. copy files 1000 to 2000.
///
/// One must provide a function that returns the progress in percent at any given time.
pub fn show_loading_animation(from: usize,
                              to: usize,
                              target: Destination,
                              progress_in_percentage_fn: &dyn Fn() -> usize) {
    if to < from { panic!("to < from") }

    let mut animation_step = 0;
    loop {
        let mut curr_percent = progress_in_percentage_fn();
        let mut curr_progress = calc_progress(from, to, curr_percent);

        // be a little bit more failure tolerant
        if curr_percent > 100 { curr_percent = 100; }
        if curr_progress > to { curr_progress = to }

        // current char of the loading animation
        let indicator = ANIMATION_STATES[animation_step];
        print_line(indicator, curr_progress, to, curr_percent, &target);

        // break when we're done
        if curr_progress >= to { break; }

        // switch to next animation step / next char
        animation_step = (animation_step + 1) % ANIMATION_STATES.len();

        sleep(Duration::from_millis(ANIMATION_SLEEP_TIMEOUT));
    }
}


/// Like show_loading_animation() but callable from C.
/// C declaration looks like this:
///
///     enum target { TARGET_STDOUT, TARGET_STDERR };
///     typedef unsigned long long usize_t; // 64 bit / usize on 64 bit machine
///     extern void show_loading_animation_ffi(usize_t, usize_t, int, usize_t (*prog_fn)());
#[no_mangle]
// pub not necessary here; this way this symbol is not visible to Rust but only to C
fn show_loading_animation_ffi(from: usize,
                              to: usize,
                              target: Destination,
                              progress_in_percentage_fn: extern "C" fn() -> usize) {
    let fn_closure = || { progress_in_percentage_fn() };
    show_loading_animation(from, to, target, &fn_closure);
}

// Prints a single line of the loading animation. Resets the previous line on each iteration by '\r'.
fn print_line(indicator: char, curr_progress: usize, to: usize, curr_percent: usize, target: &Destination) {
    // msg to be printed; \r is important to reset the current line
    let msg = format!("\r [{}] {}/{} ({}%)", indicator, curr_progress, to, curr_percent);

    if let Destination::STDOUT = &target {
        stdout().write_all(msg.as_bytes()).unwrap();
        // it's important to flush
        stdout().flush().unwrap();
    } else {
        stderr().write_all(msg.as_bytes()).unwrap();
        // it's important to flush
        stderr().flush().unwrap();
    }
}

/// Calculates the progress from the given percentage.
fn calc_progress(from: usize, to: usize, percent: usize) -> usize {
    let normalized = (to - from) as f64;
    let progress = (percent as f64) / 100_f64;
    let progress = (progress * normalized) as usize;
    let progress = progress + from;
    progress
}
