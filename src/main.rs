use clap::{App, Arg};
use rand::seq::SliceRandom;

use crate::player::Player;

mod audius;
mod player;

fn main() {
    let matches = App::new("Halkara")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hannes Braun <hannesbraun@mail.de>")
        .about("Plays the currently trending tracks on Audius")
        .arg(
            Arg::with_name("genre")
                .short("g")
                .long("genre")
                .value_name("GENRE")
                .help("Selects the trending tracks for a specified genre")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("time")
                .short("t")
                .long("time")
                .value_name("TIME")
                .help("Selects the trending tracks over a specified time range")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("order")
                .short("o")
                .long("order")
                .value_name("ORDER")
                .help("The order in which to play the trending tracks")
                .possible_values(&["asc", "desc", "rand"])
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let genre = matches.value_of("genre").unwrap_or("");
    let time = matches.value_of("time").unwrap_or("");
    let order = matches.value_of("order").unwrap_or("asc");

    let mut tracks = audius::get_trending(genre, time);

    // Reorder tracks
    match order {
        "desc" => {
            tracks.reverse();
        }
        "rand" => {
            tracks.shuffle(&mut rand::thread_rng());
        }
        _ => {}
    }

    // Create player
    let player = Player::new();

    for track in tracks {
        println!();
        println!(
            "===================================== #{:0>3} =====================================",
            track.rank
        );
        println!("Title: {}", track.track.title);
        println!("User: {}", track.track.user.name);
        println!("Duration: {}", track.track.get_duration());

        player.play(track.track);
    }
}
