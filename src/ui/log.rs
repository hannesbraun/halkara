use super::{utils::term_width, Event, HalkaraUi};
use crate::audius::TrackGroup;
use std::borrow::BorrowMut;
use std::io::{stdin, BufRead};
use std::sync::mpsc::Sender;

pub struct Log;

impl Log {
    pub fn new() -> Log {
        Log {}
    }
}

impl HalkaraUi for Log {
    fn setup(&mut self) {}

    fn start_reader(&self, sender: Sender<Event>) {
        std::thread::spawn(move || event_reader(sender));
    }

    fn display(&self, track_groups: &[TrackGroup], group: usize, track_index: usize) {
        println!();
        print_rank(track_groups[group].tracks[track_index].index);
        println!(
            "Title: {}",
            track_groups[group].tracks[track_index].track.title
        );
        println!(
            "User: {}",
            track_groups[group].tracks[track_index].track.user.name
        );
        println!(
            "Duration: {}",
            track_groups[group].tracks[track_index].track.get_duration()
        );
    }

    fn error(&self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn cleanup(&self) {}
}

fn print_rank(rank: usize) {
    let width = term_width();

    let line_char = "=";
    let rank_width = 6;
    let half_width = (width - rank_width) / 2;
    let half_line = line_char.repeat(std::cmp::max(half_width, 0) as usize);
    let filler = if half_width * 2 + rank_width != width {
        line_char
    } else {
        ""
    };

    println!("{} #{:0>3} {}{}", half_line, rank, half_line, filler);
}

pub(crate) fn event_reader(sender: Sender<Event>) {
    let mut pressed_keys = Vec::new();
    loop {
        if pressed_keys.is_empty() {
            let mut line = String::new();
            stdin()
                .lock()
                .read_line(&mut line)
                .expect("Reading line from stdin");
            pressed_keys.append(line.chars().collect::<Vec<char>>().borrow_mut());
        }

        match pressed_keys.drain(0..1).next().unwrap_or_default() {
            'q' => {
                // Quit event will be sent after loop
                break;
            }
            ' ' => {
                sender.send(Event::Pause).expect("Sending pause event");
            }
            '+' => {
                sender
                    .send(Event::VolumeUp)
                    .expect("Sending volume up event");
            }
            '-' => {
                sender
                    .send(Event::VolumeDown)
                    .expect("Sending volume down event");
            }
            _ => {}
        }
    }

    sender.send(Event::Quit).expect("Sending quit event");
}
