use super::*;

pub struct Entry {
    status: Status,
    title: Text,
    title_message: Option<Text>,
    messages: Vec<Message>,
}

impl Entry {
    pub fn new(
        status: Status,
        title: Text,
        title_message: Option<Text>,
        messages: Vec<Message>,
    ) -> Self {
        let title = title.into();
        let title_message = match title_message {
            Some(m) => Some(m.into()),
            None => None,
        };
        Self {
            status,
            title,
            title_message,
            messages,
        }
    }

    pub fn padding() -> usize {
        5
    }

    fn print_header(&self, width: usize, position: Position) {
        let left = match position {
            Position::First => "┌",
            Position::Middle => "├",
            Position::Last => {
                if self.messages.is_empty() {
                    "└"
                } else {
                    "├"
                }
            }
        };
        let right = match position {
            Position::First => "─┐",
            Position::Middle => "─┤",
            Position::Last => {
                if self.messages.is_empty() {
                    "─┘"
                } else {
                    "─┤"
                }
            }
        };
        let mut w: usize = 1 + Self::padding() + 2 + 2 + 2;

        let mut s = left.to_string();
        // Padding
        for _ in 0..Self::padding() {
            s.push_str("─");
        }
        s.push_str("┤");
        print!(
            "{}{}{}{} ",
            Pretty::border_style(),
            Pretty::border_color(),
            s,
            termion::style::Reset,
        );

        // Title
        w += self.title.len();
        self.title.print();
        if let Some(message) = &self.title_message {
            w += message.len() + 2;
            print!(": ");
            message.print();
        }

        let mut s = " ├".to_string();
        for _ in 0..width - w {
            s.push_str("─");
        }

        print!(
            "{}{}{}{}",
            Pretty::border_style(),
            Pretty::border_color(),
            s,
            termion::style::Reset,
        );
        if position == Position::Last || !self.messages.is_empty() {
            println!("{}", self.status.as_str());
        } else {
            println!(
                "{}{}{}{}",
                Pretty::border_style(),
                Pretty::border_color(),
                right,
                termion::style::Reset,
            );
        }
    }

    fn print_botton(width: usize, position: Position) {
        let left = match position {
            Position::Last => "└",
            _ => "├",
        };
        let right = match position {
            Position::Last => "┘",
            _ => "┤",
        };
        let mut s = left.to_string();
        for _ in 0..width - 2 {
            s.push_str("─");
        }
        s.push_str(right);
        println!(
            "{}{}{}{}",
            Pretty::border_style(),
            Pretty::border_color(),
            s,
            termion::style::Reset,
        );
    }

    pub fn print(&self, width: usize, position: Position) {
        self.print_header(width, position);
        if !self.messages.is_empty() {
            for message in self.messages.iter() {
                message.print(width);
            }
        }
        if position == Position::Last {
            Self::print_botton(width, position);
        }
    }
}
