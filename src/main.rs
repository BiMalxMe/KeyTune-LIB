mod audio;
mod key_code;

use std::sync::{Arc, Mutex};
use rdev::{listen, Event, EventType};
use audio::AudioEngine;
use key_code::key_code::code_from_key;

fn main() {
    let engine = Arc::new(Mutex::new(AudioEngine::new(10)));

    println!("Listening for key presses. Press any key to play its sound...");

    let engine_clone = engine.clone();

    listen(move |event: Event| {
        if let EventType::KeyPress(key) = event.event_type {
            if let Some(code) = code_from_key(key) {
                let mut engine = engine_clone.lock().unwrap();
                engine.play_sound_for_code(code);
            }
        }
    })
    .unwrap();
}
