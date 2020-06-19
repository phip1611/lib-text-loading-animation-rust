use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};

const ANIMATION_STEPS_PER_SECOND: u64 = 4;
const ANIMATION_SLEEP_TIMEOUT: u64 = 1000 / ANIMATION_STEPS_PER_SECOND;

/// The chars for the animation (the loading spinner).
static ANIMATION_STATES: [char; 4] = ['—', '\\', '|', '/'];

/// Displays a loading animation on stdout. From and To are both values that are not necessarily
/// percent. They describe to problem range, e.g. copy files 1000 to 2000.
///
/// One must provide a function that returns the progress in percent at any given time.
#[no_mangle]
pub fn show_loading_animation(from: usize,
                              to: usize,
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
        let indicator = format!("{}", ANIMATION_STATES[animation_step]);

        // msg to be printed; \r is important to reset the current line
        let msg = format!("\r [{}] {}/{} ({}%)", indicator.as_str(), curr_progress, to, curr_percent);

        stdout().write(msg.as_bytes()).unwrap();
        // it's important to flush
        stdout().flush().unwrap();

        // break when we're done
        if curr_progress >= to { break; }

        // switch to next animation step / next char
        animation_step = (animation_step + 1) % ANIMATION_STATES.len();

        sleep(Duration::from_millis(ANIMATION_SLEEP_TIMEOUT));
    }
}

/// Calculates the progress from the given percentage.
fn calc_progress(from: usize, to: usize, percent: usize) -> usize {
    let normalized = (to - from) as f64;
    let progress = (percent as f64) / 100_f64;
    let progress = (progress * normalized) as usize ;
    let progress = progress + from;
    progress
}
