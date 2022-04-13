use std::time::Duration;

use rand::seq::SliceRandom;

use crate::player::Player;
use crate::ui::HalkaraUi;

mod args;
mod audius;
mod player;
mod ui;
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
        console_args.time.clone().unwrap_or_default().as_str(),
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

    let mut hui: Box<dyn HalkaraUi>;
    #[cfg(feature = "ncurses")]
    {
        if console_args.log {
            hui = Box::new(ui::log::Log::new());
        } else {
            let header_line = match console_args
                .time
                .unwrap_or_else(|| "week".to_string())
                .to_lowercase()
                .as_str()
            {
                "week" => String::from("Trending tracks of this week"),
                "month" => String::from("Trending tracks of this month"),
                "alltime" => String::from("Trending tracks of all time"),
                &_ => String::from("Trending tracks of... I don't know?!"),
            };

            hui = Box::new(ui::ncurses::Ncurses::new(header_line));
        }
    }
    #[cfg(not(feature = "ncurses"))]
    {
        hui = Box::new(ui::log::Log::new());
    }

    hui.setup();
    for track in tracks {
        hui.display(&track);
        player.play(track.track);
    }
    hui.cleanup();
}
