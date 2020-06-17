use text_loading_animation::show_loading_animation;
use std::time::{Instant, Duration};
use std::ops::Add;

fn main() {
    let begin = Instant::now();
    let end = begin.add(Duration::from_secs(10));
    // normalize to begin is zero
    let end_millis_normalized = (end - begin).as_millis() as f64;

    let long_running_task = move || {
        let now = Instant::now();
        // normalize it to begin is zero as well
        let curr_millis_normalized = (now - begin).as_millis() as f64;
        let percent = curr_millis_normalized / end_millis_normalized * 100_f64;
        let percent = percent as usize;
        if percent > 100 { 100 } else {
            percent as usize
        }
    };

    show_loading_animation(150, 1003, &long_running_task)
}
