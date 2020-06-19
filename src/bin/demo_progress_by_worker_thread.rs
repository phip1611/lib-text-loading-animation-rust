use text_loading_animation::show_loading_animation;
use std::time::{Duration};
use std::sync::{Mutex, Arc};
use std::thread::{spawn, sleep};

fn main() {
    // Our shared progress counter; worker-thread can notify main thread about progress this way
    let progress = Arc::from(Mutex::from(0 as usize));
    // copy for worker thread
    let progress_t = progress.clone();
    let h = spawn(move || {
        loop {
            {
                let mut val = progress_t.lock().unwrap();
                if *val > 100 { break }
                *val = *val + 1;
            }
            // release look before sleep
            sleep(Duration::from_millis(50));
        }
    });

    // prepare function
    let get_progress_fn = move || {
        let progress = progress.lock().unwrap();
        *progress
    };

    show_loading_animation(0, 100, &get_progress_fn);

    // gracefully shut down thread; even tho it should be dead by know
    h.join().unwrap();

    // Overwrite last written line from loading animation
    println!("\rdone :)                     ");
}
