use crate::audius::TrendingTrack;

pub mod log;

#[cfg(feature = "ncurses")]
pub mod ncurses;

pub trait HalkaraUi {
    fn setup(&mut self);
    fn display(&self, track: &TrendingTrack);
    fn cleanup(&self);
}
