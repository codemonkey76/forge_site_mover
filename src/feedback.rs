use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

use crate::error::{AppError, AppResult};

pub fn show_spinner<F, T>(task: F, message: &str) -> AppResult<T>
where
    F: FnOnce() -> AppResult<T> + Send + 'static,
    T: Send + 'static,
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
    print!("\r"); // Clear spinner line
    io::stdout().flush().unwrap();

    // Ensure the task finishes
    match handle.join() {
        Ok(inner_result) => match inner_result {
            Ok(result) => {
                println!("\r✔  {} - complete", message);
                Ok(result)
            }
            Err(e) => {
                eprintln!("✖  {} - failed: {}", message, e);
                Err(e) // Task failed, propagate error
            }
        },
        Err(_) => {
            eprintln!("✖  {} - failed: thread panicked", message);
            Err(AppError::CommandError(
                "show_spinner".into(),
                std::io::Error::new(std::io::ErrorKind::Other, "Thread panicked"),
            ))
        }
    }
}
