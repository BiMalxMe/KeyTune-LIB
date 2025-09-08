use std::{fs::File, io::BufReader};
use rodio_wav_fix::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use rand::Rng;

pub struct AudioEngine {
    sinks: Vec<Sink>,
    stream_handle: OutputStreamHandle,
    _stream: OutputStream,
    max_concurrent: usize,
    total_sounds: usize,
}

impl AudioEngine {
    pub fn new(max_concurrent: usize, total_sounds: usize) -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sinks = Vec::with_capacity(max_concurrent);

        AudioEngine {
            sinks,
            stream_handle,
            _stream,
            max_concurrent,
            total_sounds,
        }
    }

    pub fn play_random_sound(&mut self) {
        let mut rng = rand::thread_rng();
        let file_number = rng.gen_range(1..=self.total_sounds);
        let file_path = format!("sounds/{}.ogg", file_number);

        let file = BufReader::new(File::open(&file_path).unwrap());
        let decoder = Decoder::new(file).unwrap().buffered();

        let sink = Sink::try_new(&self.stream_handle).unwrap();
        sink.append(decoder);

        self.sinks.push(sink);

        if self.sinks.len() > self.max_concurrent {
            self.sinks.remove(0);
        }
    }
}
