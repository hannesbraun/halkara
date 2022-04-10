use crate::PlayOrder;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::time::Duration;

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

trait PicoParsable<T> {
    fn pico_parse(str: &str) -> Result<T, ParseArgError>;
}

impl PicoParsable<PlayOrder> for PlayOrder {
    fn pico_parse(str: &str) -> Result<PlayOrder, ParseArgError> {
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

use std::num::ParseIntError;
impl PicoParsable<Duration> for Duration {
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

pub struct ConsoleArgs {
    pub(crate) genre: Option<String>,
    pub(crate) max_length: Option<Duration>,
    pub(crate) min_length: Option<Duration>,
    pub(crate) order: PlayOrder,
    pub(crate) time: Option<String>,
    pub(crate) volume: f32,
}

pub fn handle_args() -> Option<ConsoleArgs> {
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
        .opt_value_from_fn(["-o", "--order"], PlayOrder::pico_parse)
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
        return None;
    }

    if version {
        println!("halkara {}", env!("CARGO_PKG_VERSION"));
        return None;
    }

    Some(ConsoleArgs {
        genre,
        min_length,
        max_length,
        order,
        time,
        volume,
    })
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
