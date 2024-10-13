use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::error::AppResult;

pub fn show_spinner<F>(task: F, message: &str)
where
    F: FnOnce() -> AppResult<()> + Send + 'static,
{
    let spinner_frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let spinner_len = spinner_frames.len();

    let handle = thread::spawn(task);

    let mut i = 0;
    while !handle.is_finished() {
        print!("\r{} {}...", spinner_frames[i % spinner_len], &message);
        io::stdout().flush().unwrap();
        i += 1;
        thread::sleep(Duration::from_millis(100)); // Adjust speed as needed
    }

    // Clean up the spinner
    println!("\r✔  {} - complete", message);
    io::stdout().flush().unwrap();

    // Ensure the task finishes
    let _ = handle.join();
}
