use std::io::Cursor;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::audius::track;

pub struct Player {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl Player {
    pub fn new() -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Player {
            _stream,
            _stream_handle : stream_handle,
            sink,
        }
    }

    pub fn play(&self, track: track::Track) {
        let stream = track.get_stream();
        let cursor = Cursor::new(stream);
        let decoder = Decoder::new(cursor).unwrap();
        self.sink.append(decoder);
        self.sink.sleep_until_end();
    }
}
