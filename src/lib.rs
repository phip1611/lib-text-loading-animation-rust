use std::thread::sleep;
use std::time::Duration;
use std::io::{stdout, Write};

/// The chars for the animation (the loading spinner).
static ANIMATION_STATES: [char; 4] = ['—', '\\', '|', '/'];

/// Displays a loading animation on stdout. From and To are both values that are not necessarily
/// percent. They describe to problem range, e.g. copy files 1000 to 2000.
///
/// One need to provide a function that returns the progress in percent at any given time.
pub fn show_loading_animation(from: usize,
                              to: usize,
                              progress_in_percentage_fn: Box<dyn Fn() -> usize>) {
    if to < from { panic!("to < from") }

    let mut animation_step = 0;
    loop {
        let curr_percent = progress_in_percentage_fn();
        let curr_progress = calc_progress(from, to, curr_percent);

        // current char of the loading animation
        let indicator = format!("{}", ANIMATION_STATES[animation_step]);

        let msg = format!("\r [{}] {}/{} ({}%)", indicator.as_str(), curr_progress, to, curr_percent);

        stdout().write(msg.as_bytes()).unwrap();
        // it's important to flush
        stdout().flush().unwrap();

        // break when we're done
        if curr_progress >= to { break; }

        animation_step = (animation_step + 1) % ANIMATION_STATES.len();
        // 4 changes/animation steps per second
        sleep(Duration::from_millis(1000 / 4));
    }
}

fn calc_progress(from: usize, to: usize, percent: usize) -> usize {
    let normalized = (to - from) as f64;
    let progress = (percent as f64) / 100_f64;
    let progress = (progress * normalized) as usize ;
    let progress = progress + from;
    progress
}
