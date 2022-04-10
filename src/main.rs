use std::time::Duration;

use rand::seq::SliceRandom;
use terminal_size::{terminal_size, Width};

use crate::player::Player;

mod args;
mod audius;
mod player;
mod utils;

enum PlayOrder {
    Ascending,
    Descending,
    Random,
}

fn main() {
    let console_args = unwrap_or_return!(args::handle_args());

    let mut tracks = audius::get_trending(
        &console_args.genre.unwrap_or_default(),
        &console_args.time.unwrap_or_default(),
    );

    // Filter tracks
    tracks = tracks
        .into_iter()
        .filter(|t| {
            Duration::from_secs(t.track.duration as u64)
                <= console_args
                    .max_length
                    .unwrap_or(Duration::from_secs(u64::MAX))
        })
        .filter(|t| {
            Duration::from_secs(t.track.duration as u64)
                >= console_args
                    .min_length
                    .unwrap_or(Duration::from_secs(u64::MIN))
        })
        .collect();

    // Reorder tracks
    match console_args.order {
        PlayOrder::Descending => {
            tracks.reverse();
        }
        PlayOrder::Random => {
            tracks.shuffle(&mut rand::thread_rng());
        }
        _ => {}
    }

    // Create player
    let player = Player::new(console_args.volume);

    for track in tracks {
        println!();
        print_rank(track.rank);
        println!("Title: {}", track.track.title);
        println!("User: {}", track.track.user.name);
        println!("Duration: {}", track.track.get_duration());

        player.play(track.track);
    }
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
