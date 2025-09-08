use rodio_wav_fix::{Decoder, OutputStream, Sink, Source};
use rodio_wav_fix::source::Buffered;
use std::fs::File;
use std::io::BufReader;

pub struct AudioEngine {
    sounds: Vec<Buffered<Decoder<BufReader<File>>>>,
    rr_index: usize,
    volume: f32,
}

impl AudioEngine {
    pub fn new(volume: f32) -> anyhow::Result<Self> {
        Ok(Self {
            sounds: Vec::new(),
            rr_index: 0,
            volume,
        })
    }

    pub fn load_folder(&mut self, path: &str) -> anyhow::Result<()> {
        let entries = std::fs::read_dir(path)?;
        for entry in entries {
            let path = entry?.path();
            if path.extension().map(|s| s == "ogg").unwrap_or(false) {
                let file = BufReader::new(File::open(&path)?);
                let decoder = Decoder::new(file)?;
                self.sounds.push(decoder.buffered());
            }
        }
        Ok(())
    }

    pub fn play_round_robin(&mut self) {
        if self.sounds.is_empty() {
            return;
        }
        let sound = &self.sounds[self.rr_index];
        self.rr_index = (self.rr_index + 1) % self.sounds.len();

        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.set_volume(self.volume);
            sink.append(sound.clone());
            sink.detach();
        }
    }

    pub fn play_random(&mut self) {
        use rand::{thread_rng, Rng}; // fixed imports
        if self.sounds.is_empty() {
            return;
        }
        let idx = thread_rng().gen_range(0..self.sounds.len()); // use gen_range
        let sound = &self.sounds[idx];

        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            let sink = Sink::try_new(&stream_handle).unwrap();
            sink.set_volume(self.volume);
            sink.append(sound.clone());
            sink.detach();
        }
    }
}
