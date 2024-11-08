use std::{sync::{Arc, Mutex}, thread, time::Duration};

const NUM_BARS: usize = 4;
const MAX_PROGRESS: usize = 10;

// ANSI escape codes for colors
const COLORS: [&str; 4] = [
    "\x1b[31m",  // Red
    "\x1b[32m",  // Green
    "\x1b[33m",  // Yellow
    "\x1b[34m",  // Blue
];

fn main() {
    let progress = Arc::new(Mutex::new(vec![0; NUM_BARS])); // Shared progress state

    let mut handles = vec![];

    // Spawn threads for each progress bar
    for i in 0..NUM_BARS {
        let progress_clone = Arc::clone(&progress);
        let color = COLORS[i];  // Pick a color for this thread

        let handle = thread::spawn(move || {
            for _ in 0..MAX_PROGRESS {
                thread::sleep(Duration::from_millis(100)); // Simulate work
                let mut progress = progress_clone.lock().unwrap();
                progress[i] += 1;

                // Print progress with color
                let bar: String = "=".repeat(progress[i]) + &" ".repeat(MAX_PROGRESS - progress[i]);
                println!("{}[Bar {}] [{}] {}{}", color, i + 1, bar, progress[i], "\x1b[0m");
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    println!("All progress bars completed!");
}

