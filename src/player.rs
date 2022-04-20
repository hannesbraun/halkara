use std::io::Cursor;
use std::str;
use std::sync::mpsc::Sender;
use std::sync::{Arc, RwLock};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use crate::audius::track;
use crate::Event;

pub struct Player {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    event_sender: Sender<Event>,
    sink: Arc<RwLock<Sink>>,
}

impl Player {
    pub fn new(event_sender: Sender<Event>, volume: f32) -> Player {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        if volume != 0.0 {
            let lin = 10.0f32.powf(volume / 10.0f32);
            sink.set_volume(lin);
        }

        Player {
            _stream,
            _stream_handle: stream_handle,
            event_sender,
            sink: Arc::new(RwLock::new(sink)),
        }
    }

    pub fn play(&self, track: &track::Track) -> Result<(), String> {
        let stream = match track.get_stream() {
            Ok(stream) => stream,
            Err(e) => return Err(e.to_string()),
        };
        let error_msg = if stream.len() < 16384 {
            str::from_utf8(&stream).unwrap_or("Error: invalid stream format")
        } else {
            "Error: invalid stream format"
        }
        .to_string();

        let cursor = Cursor::new(stream);
        let decoder = Decoder::new(cursor);
        match decoder {
            Ok(decoder) => match self.sink.read() {
                Ok(sink) => {
                    sink.append(decoder);
                    self.sleep_until_end();
                    Ok(())
                }
                Err(e) => Err(e.to_string()),
            },
            Err(_) => Err(error_msg),
        }
    }

    pub fn pause(&self) {
        if let Ok(sink) = self.sink.read() {
            if sink.is_paused() {
                sink.play();
            } else {
                sink.pause();
            }
        }
    }

    const VOLUME_ADJUST: f32 = 0.69;

    pub fn volume_up(&self) {
        if let Ok(sink) = self.sink.read() {
            let vol = sink.volume() / Player::VOLUME_ADJUST;
            if vol <= 1.0 {
                sink.set_volume(vol);
            }
        }
    }

    pub fn volume_down(&self) {
        if let Ok(sink) = self.sink.read() {
            sink.set_volume(Player::VOLUME_ADJUST * sink.volume());
        }
    }

    fn sleep_until_end(&self) {
        let event_sender = self.event_sender.clone();
        let sink = self.sink.clone();
        std::thread::spawn(move || {
            if let Ok(sink) = sink.read() {
                sink.sleep_until_end();
                event_sender
                    .send(Event::TrackEnd)
                    .expect("Sending track end event");
            }
        });
    }
}
