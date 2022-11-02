use super::*;

pub struct Message {
    title: Option<Text>,
    message: Text,
}

impl Message {
    pub fn new(title: Option<Text>, message: Text) -> Self {
        Self { title, message }
    }

    pub fn padding() -> usize {
        1
    }

    fn print_sep() {
        print!(
            "{}{}│{}",
            Pretty::border_style(),
            Pretty::border_color(),
            termion::style::Reset,
        );
    }

    fn print_padding() {
        for _ in 0..Self::padding() {
            print!(" ");
        }
    }
    fn print_message(&self, text: &str) {
        print!(
            "{}{}{}{}",
            self.message.style(),
            self.message.color(),
            text,
            termion::style::Reset
        );
    }
    fn fill(n: usize) {
        for _ in 0..n {
            print!(" ");
        }
    }

    pub fn print(&self, width: usize) {
        let mut text = "".to_string();

        if let Some(title) = &self.title {
            text.push_str(&format!("{}: ", title.text()));
        }
        text.push_str(self.message.text());

        let w = width - 2 - 2 * Self::padding();
        let mut lines = textwrap::wrap(&text, w);

        if let Some(title) = &self.title {
            let first = lines.first().unwrap()[title.len() + 2..].to_string();

            Self::print_sep();
            Self::print_padding();
            title.print();
            print!(": ");
            self.print_message(first.as_str());
            Self::print_padding();
            Self::fill(w - title.len() - 2 - first.len());
            Self::print_sep();

            lines.remove(0);
        }

        for line in lines {
            let line = line.to_string();
            Self::print_sep();
            Self::print_padding();
            self.print_message(line.as_str());
            Self::print_padding();
            Self::fill(w - line.len());
            Self::print_sep();
        }

        println!();
    }
}
