use std::sync::mpsc::{channel, Sender};
use anyhow::Result;
use rdev::{listen, EventType};

mod audio;
mod key_code;

use audio::AudioEngine;

fn main() -> Result<()> {
    // Channel to send play requests
    let (tx, rx) = channel::<()>();

    // Spawn audio thread
    std::thread::spawn(move || {
        let mut engine = AudioEngine::new(10, 116);

        for _ in rx {
            engine.play_random_sound();
        }
    });

    println!("Listening for key presses. Press any key to play sound...");

    // Listen for key presses in main thread
    listen(move |event| {
        if let EventType::KeyPress(_) = event.event_type {
            // Send request to audio thread
            let _ = tx.send(());
        }
    })
    .unwrap();

    Ok(())
}
