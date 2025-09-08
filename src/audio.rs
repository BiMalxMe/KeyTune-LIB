use std::{fs::File, io::BufReader, path::Path};
use rodio_wav_fix::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

pub struct AudioEngine {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sinks: Vec<Sink>,
    max_concurrent: usize,
}

impl AudioEngine {
    pub fn new(max_concurrent: usize) -> Self {
        let (stream, stream_handle) =
            OutputStream::try_default().expect("Failed to get default output stream");

        Self {
            stream,
            stream_handle,
            sinks: Vec::new(),
            max_concurrent,
        }
    }

    pub fn play_sound_for_code(&mut self, code: i32) {
        let file_path = format!("sounds/{}.ogg", code);
        let path = Path::new(&file_path);

        if !path.exists() {
            eprintln!("Sound file not found: {:?}", path);
            return;
        }

        let file = match File::open(path) {
            Ok(f) => BufReader::new(f),
            Err(e) => {
                eprintln!("Failed to open file {:?}: {}", path, e);
                return;
            }
        };

        let decoder = match Decoder::new(file) {
            Ok(d) => d.buffered(),
            Err(e) => {
                eprintln!("Failed to decode audio {:?}: {}", path, e);
                return;
            }
        };

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(decoder);
        self.sinks.push(sink);

        if self.sinks.len() > self.max_concurrent {
            self.sinks.remove(0);
        }
    }
}
