use super::*;
use terminal_size::{terminal_size, Height, Width};

pub struct Pretty {
    entries: Vec<Entry>,
}

impl Pretty {
    pub fn new() -> Self {
        Self { entries: vec![] }
    }

    pub fn add(&mut self, entriy: Entry) {
        self.entries.push(entriy);
    }

    pub fn width() -> usize {
        let size = terminal_size();
        if let Some((Width(w), Height(_))) = size {
            w as usize
        } else {
            80
        }
    }

    pub fn border_style() -> &'static str {
        termion::style::Reset.as_ref()
    }
    pub fn border_color() -> &'static str {
        termion::color::White.fg_str()
    }

    pub fn print(&self) {
        let width = Self::width();

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

        if let Some((first, others)) = self.entries.split_first() {
            first.print(width, true, others.is_empty());

            if let Some((last, others)) = others.split_last() {
                for entry in others {
                    entry.print(width, false, false);
                }
                last.print(width, false, true);
            }
        } else {
            println!();
        }
    }
}
