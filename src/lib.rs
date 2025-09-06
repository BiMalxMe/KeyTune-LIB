// src/lib.rs

/// Initialize audio output (later we'll use rodio).
pub fn init_audio() {
    // Placeholder for setting up the audio device
    println!("[lib] Audio initialized");
}

/// Play a click sound (later this will load and play .wav file).
pub fn play_click() {
    println!("[lib] Playing click sound");
}

/// Listen for global key events (later we'll use rdev).
pub fn listen_mode() {
    println!("[lib] Listening for keypresses...");
    // TODO: call play_click() on each event
}

/// Keyword detection mode (later we’ll check input against a keyword).
pub fn keyword_mode(keyword: &str) {
    println!("[lib] Watching for keyword: {}", keyword);
    // TODO: when typed word == keyword, call play_click()
}

/// Test function for sanity check.
pub fn test_sound() {
    println!("[lib] Test sound trigger");
    play_click();
}
