use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{self, Event};
use std::time::Duration;

/// Enable raw mode (equivalent to disable_input_buffering in C)
pub fn disable_input_buffering() {
    enable_raw_mode().expect("Failed to enable raw mode");
}

/// Restore normal terminal mode (equivalent to restore_input_buffering in C)
pub fn restore_input_buffering() {
    disable_raw_mode().expect("Failed to disable raw mode");
}

/// Check if a key has been pressed (non-blocking)
pub fn check_key() -> bool {
    event::poll(Duration::from_millis(0)).unwrap_or(false)
}