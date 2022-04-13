use terminal_size::{terminal_size, Width};

use crate::audius::TrendingTrack;
use crate::ui::HalkaraUi;

pub struct Log;

impl Log {
    pub fn new() -> Log {
        Log {}
    }
}

impl HalkaraUi for Log {
    fn setup(&mut self) {}

    fn display(&self, track: &TrendingTrack) {
        println!();
        print_rank(track.rank);
        println!("Title: {}", track.track.title);
        println!("User: {}", track.track.user.name);
        println!("Duration: {}", track.track.get_duration());
    }

    fn cleanup(&self) {}
}

fn print_rank(rank: u8) {
    let term_size = terminal_size();
    let width = if let Some((Width(w), _)) = term_size {
        w
    } else {
        80
    };

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
