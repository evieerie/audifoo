use std::{
    fs::File,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use rodio::{source::SeekError, Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Streamer {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Arc<Mutex<Sink>>,
}

#[allow(dead_code)]
impl Streamer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Ok(Self {
            _stream,
            _stream_handle: stream_handle,
            sink: Arc::new(Mutex::new(sink)),
        })
    }

    pub fn play(&self) {
        let sink = self.sink.lock().unwrap();
        if sink.empty() {
            println!("sink is empty");
            return;
        }
        sink.play();
    }

    pub fn pause(&self) {
        let sink = self.sink.lock().unwrap();
        if sink.empty() {
            println!("sink is empty");
            return;
        }
        sink.pause();
    }

    pub fn skip(&self) {
        let sink = self.sink.lock().unwrap();
        sink.skip_one();
    }

    pub fn scrub(&self, pos: Duration) -> Result<(), SeekError> {
        let sink = self.sink.lock().unwrap();
        return sink.try_seek(pos);
    }

    pub fn set_volume(&self, volume: f32) {
        let sink = self.sink.lock().unwrap();
        sink.set_volume(volume);
    }

    pub fn append_to_queue(&self, path: PathBuf) {
        let file = BufReader::new(File::open(path.clone()).unwrap());
        let source = Decoder::new(file).unwrap();
        let sink = self.sink.lock().unwrap();

        sink.skip_one();
        sink.append(source);
    }
}
