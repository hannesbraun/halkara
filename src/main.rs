use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::time::Duration;

use rand::seq::SliceRandom;
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
    fn from_string(str: &str) -> Result<PlayOrder, ParseArgError> {
        match str {
            "asc" => Ok(PlayOrder::Ascending),
            "desc" => Ok(PlayOrder::Descending),
            "rand" => Ok(PlayOrder::Random),
            _ => Err(ParseArgError {
                details: str.to_owned() + " is not a valid order",
            }),
        }
    }
}

#[derive(Debug)]
struct ParseArgError {
    details: String,
}

impl Display for ParseArgError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for ParseArgError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl From<ParseIntError> for ParseArgError {
    fn from(e: ParseIntError) -> Self {
        ParseArgError {
            details: e.to_string(),
        }
    }
}

trait PicoParsable {
    fn pico_parse(str: &str) -> Result<Duration, ParseArgError>;
}

impl PicoParsable for Duration {
    fn pico_parse(str: &str) -> Result<Duration, ParseArgError> {
        // Split into strings consisting of number and unit
        let mut was_digit = true;
        let mut cur = String::new();
        let mut parts = Vec::new();
        for char in str.chars() {
            if char.is_digit(10) {
                if !was_digit {
                    parts.push(cur);
                    cur = String::new();
                }
                was_digit = true;
            } else {
                was_digit = false;
            }
            cur.push(char);
        }
        if !was_digit {
            parts.push(cur);
        }

        fn split_part(str: &str) -> (&str, &str) {
            // Split into number and unit
            let desc_index = str.chars().position(|c| !c.is_digit(10)).unwrap();
            str.split_at(desc_index)
        }

        // Calculate duration
        let mut duration = Duration::from_secs(0);
        for (num, unit) in parts.iter().map(|p| split_part(p)) {
            let num = str::parse::<u64>(num)?;
            match unit {
                "h" => {
                    duration += Duration::from_secs(num * 3600);
                }
                "m" => {
                    duration += Duration::from_secs(num * 60);
                }
                "s" => {
                    duration += Duration::from_secs(num);
                }
                _ => {
                    return Err(ParseArgError {
                        details: unit.to_owned() + " is not a valid unit",
                    });
                }
            }
        }

        Ok(duration)
    }
}

fn print_help() {
    println!(
        "USAGE:
    halkara [OPTIONS]

OPTIONS:
    -g, --genre <GENRE>      Selects the trending tracks for a specified genre
    -h, --help               Print help information
        --max-length         The maximum length for a track (longer tracks won't be played)
        --min-length         The minimum length for a track (shorter tracks won't be played)
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
    let max_length: Option<Duration> = args
        .opt_value_from_fn("--max-length", Duration::pico_parse)
        .expect("parsing max-length");
    let min_length: Option<Duration> = args
        .opt_value_from_fn("--min-length", Duration::pico_parse)
        .expect("parsing min-length");
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

    // Filter tracks
    tracks = tracks
        .into_iter()
        .filter(|t| {
            Duration::from_secs(t.track.duration as u64)
                <= max_length.unwrap_or(Duration::from_secs(u64::MAX))
        })
        .filter(|t| {
            Duration::from_secs(t.track.duration as u64)
                >= min_length.unwrap_or(Duration::from_secs(u64::MIN))
        })
        .collect();

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
