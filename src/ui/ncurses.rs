use super::{Event, HalkaraUi};
use crate::audius::TrackGroup;
use ncurses::*;
use std::cmp::max;
use std::sync::mpsc::{Receiver, Sender};

pub struct Ncurses {
    has_colors: bool,
}

const COLOR_PAIR_BORDER: i16 = 1;

impl HalkaraUi for Ncurses {
    fn setup(&mut self) {
        initscr();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
        nodelay(stdscr(), false);
        cbreak();

        self.has_colors = has_colors();
        if self.has_colors {
            start_color();
            init_pair(
                COLOR_PAIR_BORDER,
                constants::COLOR_WHITE,
                constants::COLOR_BLUE,
            );
        }

        self.update_header("Idle");
        self.update_footer("Welcome!");

        refresh();
    }

    fn start_reader(&self, _sender: Sender<Event>) {
        // Unsupported due to multithreading
        /*std::thread::spawn(move || {
            let mut key;
            loop {
                key = getch();
                match char::from_u32(key as u32).unwrap_or_default() {
                    'q' => {
                        sender.send(Event::Quit).expect("Sending quit event");
                    }
                    ' ' => {
                        sender.send(Event::Pause).expect("Sending pause event");
                    }
                    _ => {}
                }
            }
        });*/
    }

    fn display(&self, track_groups: &[TrackGroup], group: usize, track_index: usize) {
        self.update_header(&track_groups[group].name);

        let (mut maxy, mut maxx) = (0, 0);
        getmaxyx(stdscr(), &mut maxy, &mut maxx);
        let mid = maxy / 2;

        let global_track_index = track_groups[0..group]
            .iter()
            .flat_map(|group| &group.tracks)
            .count()
            + track_index;

        // Clear lines after mid
        for i in mid..maxy - 1 {
            mvclrtoeol(i, 0);
        }

        track_groups
            .iter()
            .flat_map(|group| &group.tracks)
            .enumerate()
            .for_each(|(i, track)| {
                let line = i as i32 - global_track_index as i32 + mid;
                if line > 0 && line < maxy - 1 {
                    if line == mid {
                        attron(A_BOLD());
                        if self.has_colors {
                            attron(COLOR_PAIR(COLOR_PAIR_BORDER));
                        }
                    }
                    mvclrtoeol(line, 0);
                    let mut out_without_duration = format!(
                        "{: >4} {} - {}{}",
                        track.index,
                        &track.track.user.name,
                        &track.track.title,
                        " ".repeat(maxx as usize)
                    );
                    out_without_duration.truncate(max(0, maxx - 7) as usize);
                    addstr(&out_without_duration);
                    let mut duration = format!(" {: >6}", &track.track.get_duration());
                    duration.truncate(7);
                    mvaddstr(line, maxx - 7, &duration);
                    if line == mid {
                        attroff(A_BOLD());
                        if self.has_colors {
                            attroff(COLOR_PAIR(COLOR_PAIR_BORDER));
                        }
                    }
                }
            });

        refresh();
    }

    fn error(&self, msg: &str) {
        self.update_footer(msg);
    }

    fn cleanup(&self) {
        mv(-1, 0);
        endwin();
    }
}

fn mvclrtoeol(y: i32, x: i32) {
    mv(y, x);
    clrtoeol();
}

impl Ncurses {
    pub fn new() -> Ncurses {
        Ncurses { has_colors: false }
    }

    fn update_header(&self, header_str: &str) {
        let maxx = getmaxx(stdscr());

        if self.has_colors {
            attron(COLOR_PAIR(COLOR_PAIR_BORDER));
            mv(0, 0);
            addstr(&" ".repeat(maxx as usize));
        }

        let header = format!("Halkara {} - {}", env!("CARGO_PKG_VERSION"), header_str);
        mvaddstr(0, (maxx - header.len() as i32) / 2, &header);

        if self.has_colors {
            attroff(COLOR_PAIR(COLOR_PAIR_BORDER));
        }
    }

    fn update_footer(&self, text: &str) {
        let (mut maxy, mut maxx) = (0, 0);
        getmaxyx(stdscr(), &mut maxy, &mut maxx);

        if self.has_colors {
            attron(COLOR_PAIR(COLOR_PAIR_BORDER));
            mv(maxy - 1, 0);
            addstr(&" ".repeat(maxx as usize));
        }

        mvaddstr(maxy - 1, (maxx - text.len() as i32) / 2, text);

        if self.has_colors {
            attroff(COLOR_PAIR(COLOR_PAIR_BORDER));
        }
    }
}
