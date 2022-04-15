use crate::audius::TrackGroup;

pub mod log;

#[cfg(feature = "ncurses")]
pub mod ncurses;

pub trait HalkaraUi {
    fn setup(&mut self);
    fn display(&self, track_groups: &[TrackGroup], group: usize, track_index: usize);
    fn error(&self, err: &str);
    fn cleanup(&self);
}
