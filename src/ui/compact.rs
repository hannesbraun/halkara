use super::{Event, HalkaraUi};
use crate::audius::TrackGroup;
use std::sync::mpsc::Sender;

pub struct Compact;

impl Compact {
    pub fn new() -> Compact {
        Compact {}
    }
}

impl HalkaraUi for Compact {
    fn setup(&mut self) {}

    fn start_reader(&self, sender: Sender<Event>) {
        // Reuse event reader from log UI
        std::thread::spawn(move || super::log::event_reader(sender));
    }

    fn display(&self, track_groups: &[TrackGroup], group: usize, track_index: usize) {
        let mut duration = track_groups[group].tracks[track_index].track.get_duration();
        duration.truncate(6);
        let line = format!(
            "{: >3} [{: >6}] {} - {}",
            track_groups[group].tracks[track_index].index,
            duration,
            track_groups[group].tracks[track_index].track.user.name,
            track_groups[group].tracks[track_index].track.title
        );
        println!("{}", line);
    }

    fn error(&self, msg: &str) {
        eprintln!("{}", msg);
    }

    fn cleanup(&self) {}
}
