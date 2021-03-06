use crate::ui::UiVariant;
use crate::PlayOrder;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
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

impl PicoParsable<UiVariant> for UiVariant {
    fn pico_parse(str: &str) -> Result<UiVariant, ParseArgError> {
        match str.to_lowercase().as_str() {
            "compact" => Ok(UiVariant::Compact),
            "log" => Ok(UiVariant::Log),
            "ncurses" => Ok(UiVariant::Ncurses),
            _ => Err(ParseArgError {
                details: str.to_owned() + " is not a valid UI variant",
            }),
        }
    }
}

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

pub struct TrendingPlayable {
    pub(crate) genre: Option<String>,
    pub(crate) time: Option<String>,
}

pub fn is_trending(arg: &str) -> bool {
    arg.starts_with("trending")
}

pub fn parse_trending_arg(arg: &str) -> TrendingPlayable {
    let splitted = arg.split(':').collect::<Vec<&str>>();
    let genre = if splitted.len() > 1 {
        if !splitted[1].is_empty() {
            Some(splitted[1].to_string())
        } else {
            None
        }
    } else {
        None
    };
    let time = if splitted.len() > 2 {
        if !splitted[2].is_empty() {
            Some(splitted[2].to_string())
        } else {
            None
        }
    } else {
        None
    };
    TrendingPlayable { genre, time }
}

// Log is only read when the ncurses feature is turned on
#[allow(dead_code)]
pub struct ConsoleArgs {
    pub(crate) genre: Option<String>,
    pub(crate) max_length: Option<Duration>,
    pub(crate) min_length: Option<Duration>,
    pub(crate) order: PlayOrder,
    pub(crate) playables: Vec<String>,
    pub(crate) ui: UiVariant,
    pub(crate) time: Option<String>,
    pub(crate) volume: f32,
}

pub fn handle_args() -> Option<ConsoleArgs> {
    let mut args = pico_args::Arguments::from_env();
    let genre = args
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
    let time = args
        .opt_value_from_str(["-t", "--time"])
        .expect("parsing time");
    let ui = args
        .opt_value_from_fn("--ui", UiVariant::pico_parse)
        .expect("parsing ui variant")
        .unwrap_or(UiVariant::Log);
    let version = args.contains(["-V", "--version"]);
    let volume = args
        .opt_value_from_str::<_, f32>("--volume")
        .expect("parsing volume")
        .unwrap_or_default()
        .min(0.0);

    let playables = args
        .finish()
        .into_iter()
        .map(|s| s.to_str().unwrap_or_default().to_string())
        .filter(|arg| !arg.starts_with('-'))
        .collect();

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
        playables,
        time,
        ui,
        volume,
    })
}

fn print_help() {
    println!(
        "USAGE:
    halkara [OPTIONS] [URLS]

OPTIONS:
    -g, --genre <GENRE>      Selects the trending tracks for a specified genre
    -h, --help               Print help information
        --max-length         The maximum length for a track (longer tracks won't be played)
        --min-length         The minimum length for a track (shorter tracks won't be played)
    -o, --order <ORDER>      The order in which to play the trending tracks [possible values: asc,
                             desc, rand]
    -t, --time <TIME>        Selects the trending tracks over a specified time range
        --ui <UI>            The user interface variant to use [possible values: compact, log,
                             ncurses]
    -V, --version            Print version information
        --volume <VOLUME>    The volume in dBFS"
    );
}
