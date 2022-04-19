use crate::audius::TrackGroup;
use std::sync::mpsc::Sender;

pub mod compact;
pub mod log;
mod utils;

#[cfg(feature = "ncurses")]
pub mod ncurses;

pub trait HalkaraUi {
    fn setup(&mut self);
    fn start_reader(&self, sender: Sender<Event>);
    fn display(&self, track_groups: &[TrackGroup], group: usize, track_index: usize);
    fn error(&self, err: &str);
    fn cleanup(&self);
}

pub enum UiVariant {
    Compact,
    Log,
    Ncurses,
}

#[derive(Debug)]
pub enum Event {
    Pause,
    Quit,
    TrackEnd,
}
