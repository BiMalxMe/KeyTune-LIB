mod audio;

use std::sync::Arc;
use clap::Parser;
use serde::Deserialize;
use once_cell::sync::OnceCell;
use flume::Sender;

use rdev::{listen, Event};

#[derive(Parser)]
struct Cli {
    /// path to sound folder
    #[arg(short, long, default_value = "sounds")]
    sound_dir: String,

    /// config file path
    #[arg(short, long, default_value = "config.json")]
    config: String,
}

#[derive(Deserialize, Debug)]
struct Config {
    volume: Option<f32>,
    mode: Option<String>, // "round_robin" or "random"
    max_concurrent: Option<usize>, // optional
}

static CONFIG: OnceCell<Config> = OnceCell::new();

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // read config
    let conf_text = std::fs::read_to_string(&cli.config).unwrap_or_else(|_| "{}".to_string());
    let conf: Config = serde_json::from_str(&conf_text).unwrap_or(Config {
        volume: Some(0.7),
        mode: Some("round_robin".into()),
        max_concurrent: Some(8),
    });
    CONFIG.set(conf).ok();

    let conf_ref = CONFIG.get().unwrap();

    // create AudioEngine in main thread
    let mut engine = audio::AudioEngine::new(conf_ref.volume.unwrap_or(0.7))?;
    engine.load_folder(&cli.sound_dir)?;

    // channel to send key events to worker thread
    let (tx, rx) = flume::unbounded::<rdev::Event>();

    // spawn worker thread for audio playback
    std::thread::spawn(move || {
        let mut engine = engine;
        while let Ok(evt) = rx.recv() {
            if let rdev::EventType::KeyPress(_) = evt.event_type {
                match CONFIG.get().and_then(|c| c.mode.clone()).as_deref() {
                    Some("random") => engine.play_random(),
                    _ => engine.play_round_robin(),
                }
            }
        }
    });

    let tx_arc = Arc::new(tx);
    listen(move |event: Event| {
        let _ = tx_arc.try_send(event);
    })
    .map_err(|e| anyhow::anyhow!("Failed to start global key listener: {:?}", e))?;

    Ok(())
}
