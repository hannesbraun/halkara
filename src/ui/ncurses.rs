use std::cmp::max;

use ncurses::*;

use crate::audius::TrendingTrack;
use crate::HalkaraUi;

pub struct Ncurses {
    has_colors: bool,
    header: String,
}

const COLOR_PAIR_BORDER: i16 = 1;

impl HalkaraUi for Ncurses {
    fn setup(&mut self) {
        initscr();
        keypad(stdscr(), true);
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        self.has_colors = has_colors();
        if self.has_colors {
            start_color();
            init_pair(
                COLOR_PAIR_BORDER,
                constants::COLOR_WHITE,
                constants::COLOR_BLUE,
            );
        }

        self.update_header();
        self.update_footer("Loading...");

        refresh();
    }

    fn display(&self, track: &TrendingTrack) {
        self.update_header();
        self.update_footer(&format!("#{:0>3}", track.rank));

        let maxy = getmaxy(stdscr());
        let starty = max(maxy / 2 - 1, 0);

        mvclrtoeol(starty, 3);
        addstr("Title: ");
        attron(A_BOLD());
        addstr(&track.track.title);
        attroff(A_BOLD());

        mvclrtoeol(starty + 1, 3);
        addstr("User: ");
        attron(A_BOLD());
        addstr(&track.track.user.name);
        attroff(A_BOLD());

        mvclrtoeol(starty + 2, 3);
        addstr("Duration: ");
        attron(A_BOLD());
        addstr(&track.track.get_duration());
        attroff(A_BOLD());

        refresh();
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
    pub fn new(header: String) -> Ncurses {
        Ncurses {
            has_colors: false,
            header,
        }
    }

    fn update_header(&self) {
        let maxx = getmaxx(stdscr());

        if self.has_colors {
            attron(COLOR_PAIR(COLOR_PAIR_BORDER));
            mv(0, 0);
            addstr(&" ".repeat(maxx as usize));
        }

        let header = format!("halkara {} - {}", env!("CARGO_PKG_VERSION"), self.header);
        mvaddstr(0, (maxx - header.len() as i32) / 2, &header);

        if self.has_colors {
            attroff(COLOR_PAIR(COLOR_PAIR_BORDER));
        }
    }

    fn update_footer(&self, text: &str) {
        let maxx = getmaxx(stdscr());
        let maxy = getmaxy(stdscr());

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
