use std::io::Cursor;
use std::str;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::audius::track;

pub struct Player {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl Player {
    pub fn new(volume: f32) -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        if volume != 0.0 {
            let lin = 10.0f32.powf(volume / 10.0f32);
            sink.set_volume(lin);
        }

        Player {
            _stream,
            _stream_handle: stream_handle,
            sink,
        }
    }

    pub fn play(&self, track: track::Track) {
        let stream = match track.get_stream() {
            Some(stream) => stream,
            None => return,
        };
        let error_msg = if stream.len() < 16384 {
            str::from_utf8(&stream).unwrap_or("Error: invalid stream format")
        } else {
            "Error: invalid stream format"
        }.to_string();

        let cursor = Cursor::new(stream);
        let decoder = Decoder::new(cursor);
        match decoder {
            Ok(decoder) => {
                self.sink.append(decoder);
                self.sink.sleep_until_end();
            }
            Err(_) => eprintln!("{}", error_msg),
        }
    }
}
