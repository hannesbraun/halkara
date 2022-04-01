use rand::seq::SliceRandom;
use std::error::Error;
use std::fmt::{Display, Formatter};
use terminal_size::{terminal_size, Width};

use crate::player::Player;

mod audius;
mod player;

enum PlayOrder {
    Ascending,
    Descending,
    Random,
}

impl PlayOrder {
    fn from_string(str: &str) -> Result<PlayOrder, ParseEnumError> {
        match str {
            "asc" => Ok(PlayOrder::Ascending),
            "desc" => Ok(PlayOrder::Descending),
            "rand" => Ok(PlayOrder::Random),
            _ => Err(ParseEnumError {
                details: str.to_owned() + " is not a valid order",
            }),
        }
    }
}

#[derive(Debug)]
struct ParseEnumError {
    details: String,
}

impl Display for ParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseEnumError {
    fn description(&self) -> &str {
        &self.details
    }
}

fn print_help() {
    println!(
        "USAGE:
    halkara [OPTIONS]

OPTIONS:
    -g, --genre <GENRE>      Selects the trending tracks for a specified genre
    -h, --help               Print help information
    -o, --order <ORDER>      The order in which to play the trending tracks [possible values: asc,
                             desc, rand]
    -t, --time <TIME>        Selects the trending tracks over a specified time range
    -V, --version            Print version information
        --volume <VOLUME>    The volume in dBFS"
    );
}

fn main() {
    let mut args = pico_args::Arguments::from_env();
    let genre: Option<String> = args
        .opt_value_from_str(["-g", "--genre"])
        .expect("parsing genre");
    let help = args.contains(["-h", "--help"]);
    let order: PlayOrder = args
        .opt_value_from_fn(["-o", "--order"], PlayOrder::from_string)
        .expect("parsing order")
        .unwrap_or(PlayOrder::Ascending);
    let time: Option<String> = args
        .opt_value_from_str(["-t", "--time"])
        .expect("parsing time");
    let version = args.contains(["-V", "--version"]);
    let volume: f32 = args
        .opt_value_from_str("--volume")
        .expect("parsing volume")
        .unwrap_or_default();

    if help {
        print_help();
        return;
    }

    if version {
        println!("halkara {}", env!("CARGO_PKG_VERSION"));
        return;
    }

    let mut tracks = audius::get_trending(&genre.unwrap_or_default(), &time.unwrap_or_default());

    // Reorder tracks
    match order {
        PlayOrder::Descending => {
            tracks.reverse();
        }
        PlayOrder::Random => {
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
    let half_line = line_char.repeat(std::cmp::max(half_width, 0) as usize);
    let filler = if half_width * 2 + rank_width != width {
        line_char
    } else {
        ""
    };

    println!("{} #{:0>3} {}{}", half_line, rank, half_line, filler);
}
