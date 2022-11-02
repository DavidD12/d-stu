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

    pub fn display(&self) {
        let width = Self::width();

        print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
        if self.entries.is_empty() {
            println!();
            return;
        }

        let first = self.entries.first().unwrap();
        first.display(width, Position::First);

        if self.entries.len() > 2 {
            for entriy in self.entries[1..self.entries.len() - 1].iter() {
                entriy.display(width, Position::Middle);
            }
        }

        if self.entries.len() > 1 {
            let last = self.entries.last().unwrap();
            last.display(width, Position::Last);
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Position {
    First,
    Middle,
    Last,
}
