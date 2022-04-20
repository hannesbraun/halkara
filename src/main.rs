use crate::args::{is_trending, parse_trending_arg};
use crate::player::Player;
use crate::ui::{Event, HalkaraUi, UiVariant};
use crate::utils::shuffle;
use std::sync::mpsc::channel;
use std::time::Duration;

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

    let mut track_groups = Vec::with_capacity(std::cmp::max(1, console_args.playables.len()));
    if console_args.playables.is_empty() {
        track_groups.push(audius::trending::get_trending(
            &console_args.genre.unwrap_or_default(),
            &console_args.time.unwrap_or_default(),
        ));
    } else {
        for playable in console_args.playables {
            if is_trending(&playable) {
                let trending_args = parse_trending_arg(&playable);
                track_groups.push(audius::trending::get_trending(
                    &trending_args.genre.unwrap_or_default(),
                    &trending_args.time.unwrap_or_default(),
                ));
            } else {
                track_groups
                    .append(&mut audius::resolve(&playable).expect("Building final playlist"));
            }
        }
    };

    for group in track_groups.iter_mut() {
        // Filter tracks
        group.tracks.retain(|t| {
            Duration::from_secs(t.track.duration as u64)
                <= console_args
                    .max_length
                    .unwrap_or(Duration::from_secs(u64::MAX))
        });
        group.tracks.retain(|t| {
            Duration::from_secs(t.track.duration as u64)
                >= console_args
                    .min_length
                    .unwrap_or(Duration::from_secs(u64::MIN))
        });

        // Reorder tracks
        match console_args.order {
            PlayOrder::Descending => {
                group.tracks.reverse();
            }
            PlayOrder::Random => {
                shuffle(&mut (group.tracks));
            }
            _ => {}
        }
    }

    // Create event channel
    let (event_sender, event_receiver) = channel();

    // Create player
    let player = Player::new(event_sender.clone(), console_args.volume);

    let mut hui: Box<dyn HalkaraUi>;
    #[cfg(feature = "ncurses")]
    {
        hui = match console_args.ui {
            UiVariant::Compact => Box::new(ui::compact::Compact::new()),
            UiVariant::Log => Box::new(ui::log::Log::new()),
            UiVariant::Ncurses => Box::new(ui::ncurses::Ncurses::new()),
        }
    }
    #[cfg(not(feature = "ncurses"))]
    {
        hui = match console_args.ui {
            UiVariant::Compact => Box::new(ui::compact::Compact::new()),
            UiVariant::Log => Box::new(ui::log::Log::new()),
            UiVariant::Ncurses => {
                eprintln!("Error: Halkara was built without ncurses support");
                use std::process::exit;
                exit(1);
            }
        }
    }
    hui.start_reader(event_sender);

    hui.setup();

    let mut quit = false;
    for (i, group) in track_groups.iter().enumerate() {
        for (j, track) in group.tracks.iter().enumerate() {
            hui.display(&track_groups, i, j);
            if let Err(err) = player.play(&track.track) {
                hui.error(&err);
                continue;
            }

            // Wait for input or track end
            loop {
                match event_receiver.recv().expect("Receiving event") {
                    Event::Pause => {
                        player.pause();
                    }
                    Event::Quit => {
                        quit = true;
                        break;
                    }
                    Event::TrackEnd => {
                        break;
                    }
                    Event::VolumeUp => {
                        player.volume_up();
                    }
                    Event::VolumeDown => {
                        player.volume_down();
                    }
                }
            }

            if quit {
                break;
            }
        }
        if quit {
            break;
        }
    }

    hui.cleanup();
}
