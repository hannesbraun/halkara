use cpal::{Sample, SampleFormat, StreamConfig, Stream, SampleRate};
use cpal::traits::{DeviceTrait, HostTrait};
use crossbeam_channel::{Sender, Receiver};
use itertools::Itertools;
use minimp3::{Decoder, Error, Frame};
use samplerate::{Samplerate, ConverterType};
use std::{thread, time};

use crate::audius::track;

const CHUNKSIZE: usize = 2048;

pub struct Player {
    tx: Option<Sender<(f32, f32)>>,
    channels_out: u16,
    sample_rate_out: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            tx: None,
            channels_out: 2,
            sample_rate_out: 44100,
        }
    }

    pub fn init(&mut self) -> Stream {
        let (tx, rx) = crossbeam_channel::bounded(CHUNKSIZE);
        self.tx = Some(tx);

        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config_range = supported_configs_range.next()
            .expect("no supported config?!");
        let min_sample_rate = supported_config_range.min_sample_rate().0;
        let max_sample_rate = supported_config_range.max_sample_rate().0;
        let supported_config;
        if 48000 > min_sample_rate && 48000 < max_sample_rate {
            supported_config = supported_config_range.with_sample_rate(SampleRate(48000));
        } else if min_sample_rate > 44100 {
            supported_config = supported_config_range.with_sample_rate(SampleRate(min_sample_rate));
        } else {
            supported_config = supported_config_range.with_max_sample_rate();
        }

        let sample_format = supported_config.sample_format();
        let config: StreamConfig = supported_config.into();
        self.channels_out = config.channels;
        self.sample_rate_out = config.sample_rate.0;

        let ecb = |_err| {
            // react to errors here
            eprintln!("an error occurred on the output audio stream");
        };

        let channels_out = self.channels_out;
        let stream = match sample_format {
            SampleFormat::I16 => device.build_output_stream::<i16, _, _>(&config, move |data, _| { Player::cb::<i16>(data, &rx, channels_out) }, ecb).unwrap(),
            SampleFormat::U16 => device.build_output_stream::<u16, _, _>(&config, move |data, _| { Player::cb::<u16>(data, &rx, channels_out) }, ecb).unwrap(),
            SampleFormat::F32 => device.build_output_stream::<f32, _, _>(&config, move |data, _| { Player::cb::<f32>(data, &rx, channels_out) }, ecb).unwrap()
        };

        stream
    }

    fn cb<T: Sample>(data: &mut [T], rx: &Receiver<(f32, f32)>, channels_out: u16) {
        // Receiving provided samples and writing them to the stream
        // This is using interleaved channels btw
        // Assumption: first sample of data always represents channel 0 (left channel)
        let mut src = (0.0, 0.0);
        let mut ch_num = 0;
        for sample in data.iter_mut() {
            match ch_num {
                0 => {
                    src = rx.try_recv().unwrap_or((0.0, 0.0));
                    *sample = T::from(&src.0);
                }
                1 => *sample = T::from(&src.1),
                _ => *sample = T::from(&0.0f32),
            }
            ch_num = (ch_num + 1) % channels_out;
        }
    }

    pub fn play(&self, track: track::Track) {
        let tx = (&self.tx).as_ref().unwrap().clone();
        let stream = track.get_stream();
        let mut decoder = Decoder::new(stream);

        let mut sample_rate_converter_mono = Samplerate::new(ConverterType::SincBestQuality, 48000, self.sample_rate_out, 1).unwrap();
        let mut sample_rate_converter_stereo = Samplerate::new(ConverterType::SincBestQuality, 48000, self.sample_rate_out, 2).unwrap();

        loop {
            match decoder.next_frame() {
                Ok(Frame { data, sample_rate, channels, .. }) => {
                    let mut samples: Vec<f32> = data.iter()
                        .enumerate()
                        .filter(|(i, _)| i % channels < 2) // Filter out channels other than left/right (0 or 1)
                        .map(|(_, s)| *s as f32 / 32768f32)
                        .collect();

                    // Sample rate conversion
                    if sample_rate as u32 != self.sample_rate_out {
                        let sample_rate_converter = match channels {
                            1 => &mut sample_rate_converter_mono,
                            _ => &mut sample_rate_converter_stereo
                        };
                        if sample_rate_converter.from_rate() != sample_rate as u32 {
                            sample_rate_converter.set_from_rate(sample_rate as u32);
                        }
                        samples = sample_rate_converter.process(&samples).unwrap_or_else(|_| vec![]);
                    }

                    samples.iter()
                        .tuples::<(_, _)>()
                        .for_each(|(left, right)| {
                            #[allow(unused_must_use)]
                            if channels == 1 {
                                // Expand mono signal
                                tx.send((*left, *left));
                                tx.send((*right, *right));
                            } else if self.channels_out == 1 {
                                // Stereo to mono
                                // 2 input channels guaranteed because of previous condition
                                let mono = (*left + *right) / 2.0f32;
                                tx.send((mono, mono));
                            } else {
                                // 2in, 2out or 1in, 1out
                                tx.send((*left, *right));
                            }
                        });
                }
                Err(Error::Eof) => break,
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    pub fn drain(&self) {
        // TODO this is super ugly... but it should work
        thread::sleep(time::Duration::from_millis(1000));
    }
}
