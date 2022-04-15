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

    let mut track_groups = vec![audius::get_trending(
        &console_args.genre.unwrap_or_default(),
        console_args.time.clone().unwrap_or_default().as_str(),
    )];

    // Filter tracks
    track_groups[0].tracks.retain(|t| {
        Duration::from_secs(t.track.duration as u64)
            <= console_args
                .max_length
                .unwrap_or(Duration::from_secs(u64::MAX))
    });
    track_groups[0].tracks.retain(|t| {
        Duration::from_secs(t.track.duration as u64)
            >= console_args
                .min_length
                .unwrap_or(Duration::from_secs(u64::MIN))
    });

    // Reorder tracks
    match console_args.order {
        PlayOrder::Descending => {
            track_groups[0].tracks.reverse();
        }
        PlayOrder::Random => {
            track_groups[0].tracks.shuffle(&mut rand::thread_rng());
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
            hui = Box::new(ui::ncurses::Ncurses::new(track_groups[0].name.clone()));
        }
    }
    #[cfg(not(feature = "ncurses"))]
    {
        hui = Box::new(ui::log::Log::new());
    }

    hui.setup();
    for (i, track) in track_groups[0].tracks.iter().enumerate() {
        hui.display(&track_groups, 0, i);
        if let Err(err) = player.play(&track.track) {
            hui.error(&err);
        }
    }
    hui.cleanup();
}
