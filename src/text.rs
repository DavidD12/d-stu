pub struct Text {
    text: String,
    style: String,
    color: String,
}

impl Text {
    pub fn new<S1, S2, S3>(text: S1, style: S2, color: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let text = text.into();
        let style = style.into();
        let color = color.into();
        Self { text, style, color }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn style(&self) -> &str {
        &self.style
    }

    pub fn color(&self) -> &str {
        &self.color
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn print(&self) {
        print!(
            "{}{}{}{}",
            self.style(),
            self.color(),
            self.text(),
            termion::style::Reset
        );
    }
}
