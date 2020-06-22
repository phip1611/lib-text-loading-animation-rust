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
    /// Print to STDOUT
    STDOUT,
    /// Print to STDERR
    STDERR,
}

/// Describes the semantics of the return value of the progress function.
#[derive(Debug)]
pub enum ProgressFnKind {
    /// Progress report function returns percentage from 0 to 100.
    PERCENT,
    /// Progress report function returns total progress in processed elements from total.
    PROGRESS
}

/// Displays a loading animation on stdout. From and To are both values that are not necessarily
/// percent. They describe to problem range, e.g. copy files 1000 to 2000.
///
/// One must provide a function that returns the progress in percent at any given time.
pub fn show_loading_animation(from: usize,
                              to: usize,
                              target: Destination,
                              progress_fn: &dyn Fn() -> usize,
                              fn_kind: ProgressFnKind) {
    assert!(from <= to);

    let mut animation_step = 0;
    loop {
        let (curr_percent, curr_progress) = get_curr_prog_n_perc(progress_fn, &fn_kind, from, to);

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
///     /** The target stream for the loading animation. */
///     enum target { TARGET_STDOUT, TARGET_STDERR };
///     /** The semantic type of the progress function. */
///     enum fn_kind { PERCENTAGE_FN, PROGRESS_FN };
///     typedef unsigned long long usize_t; // 64 bit / usize on 64 bit machine
///     extern void show_loading_animation_ffi(usize_t, usize_t, int, usize_t (*prog_fn)(), int);
#[no_mangle]
// pub not necessary here; this way this symbol is not visible to Rust but only to C
fn show_loading_animation_ffi(from: usize,
                              to: usize,
                              target: Destination,
                              progress_in_percentage_fn: extern "C" fn() -> usize,
                              fn_kind: ProgressFnKind) {
    let fn_closure = || { progress_in_percentage_fn() };
    show_loading_animation(from, to, target, &fn_closure, fn_kind);
}

/// Calculates the current percentage and total current progress of the workload for
/// the loading animation.
fn get_curr_prog_n_perc(fnc: &dyn Fn() -> usize,
                        fn_kind: &ProgressFnKind,
                        from: usize,
                        to: usize) -> (usize, usize) {
    let mut percentage: usize;
    let mut total_current_progress: usize;
    if let ProgressFnKind::PERCENT = fn_kind {
        percentage = fnc();
        if percentage > 100 {
            percentage = 100;
        }
        total_current_progress = calc_progress(from, to, percentage);
    } else {
        total_current_progress = fnc();
        if total_current_progress > to {
            total_current_progress = to;
        }
        percentage = calc_percentage(from, to, total_current_progress);
    }
    (percentage, total_current_progress)
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
    // not necessary; usize assert!(0 <= percent);
    assert!(percent <= 100);
    let normalized = (to - from) as f64;
    let progress = (percent as f64) / 100_f64;
    let progress = (progress * normalized) as usize;
    let progress = progress + from;
    progress
}

/// Calculates the percentage of the given progress.
fn calc_percentage(from: usize, to: usize, progress: usize) -> usize {
    assert!(from <= progress);
    assert!(progress <= to);
    let normalized_to = to - from;
    if normalized_to == 0 { return 100 }

    let normalized_to = normalized_to as f64;
    let normalized_progress = (progress - from) as f64;
    ((normalized_progress / normalized_to) * 100_f64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_progress() {
        assert_eq!(0, calc_progress(0, 100, 0));
        assert_eq!(5, calc_progress(0, 100, 5));
        assert_eq!(100, calc_progress(0, 100, 100));

        assert_eq!(0 + 10, calc_progress(10, 20, 0));
        assert_eq!(1 + 10, calc_progress(10, 20, 10));
        assert_eq!(10 + 10, calc_progress(10, 20, 100));

        assert_eq!(0, calc_progress(0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn test_calc_progress_panic_1() {
        calc_progress(0, 100, 101);
    }

    #[test]
    fn test_calc_percentage() {
        assert_eq!(0, calc_percentage(0, 100, 0));
        assert_eq!(20, calc_percentage(0, 100, 20));
        assert_eq!(100, calc_percentage(0, 100, 100));

        assert_eq!(0, calc_percentage(10, 20, 10));
        assert_eq!(50, calc_percentage(10, 20, 15));
        assert_eq!(100, calc_percentage(10, 20, 20));

        // 0 from 0 is like 100 percent
        assert_eq!(100, calc_percentage(0, 0, 0));
    }

    #[test]
    #[should_panic]
    fn test_calc_percentage_panic_1() {
        calc_percentage(20, 100, 19);
    }

    #[test]
    #[should_panic]
    fn test_calc_percentage_panic_2() {
        calc_percentage(20, 100, 101);
    }
}
