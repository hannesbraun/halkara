use clap::{Arg, Command};
use rand::seq::SliceRandom;
use terminal_size::{terminal_size, Width};

use crate::player::Player;

mod audius;
mod player;

fn main() {
    let matches = Command::new("halkara")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Hannes Braun <hannesbraun@mail.de>")
        .about("Plays the currently trending tracks on Audius")
        .arg(
            Arg::new("genre")
                .short('g')
                .long("genre")
                .value_name("GENRE")
                .help("Selects the trending tracks for a specified genre")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("time")
                .short('t')
                .long("time")
                .value_name("TIME")
                .help("Selects the trending tracks over a specified time range")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("order")
                .short('o')
                .long("order")
                .value_name("ORDER")
                .help("The order in which to play the trending tracks")
                .possible_values(&["asc", "desc", "rand"])
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .multiple_occurrences(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::new("volume")
                .long("volume")
                .value_name("VOLUME")
                .help("The volume in dBFS")
                .takes_value(true)
                .required(false)
                .allow_hyphen_values(true),
        )
        .get_matches();

    let genre = matches.value_of("genre").unwrap_or("");
    let time = matches.value_of("time").unwrap_or("");
    let order = matches.value_of("order").unwrap_or("asc");
    let volume = matches
        .value_of("volume")
        .unwrap_or("0.0")
        .parse::<f32>()
        .unwrap_or(0.0);

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
    let player = Player::new(volume);

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
    let half_line = std::iter::repeat(line_char)
        .take(half_width as usize)
        .collect::<String>();
    let filler = if half_width * 2 + rank_width != width {
        line_char
    } else {
        ""
    };

    println!("{} #{:0>3} {}{}", half_line, rank, half_line, filler);
}
