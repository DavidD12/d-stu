pub enum Status {
    Running,
    Success,
    Failure,
    Info,
    Question,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Running => "⏳",
            Status::Success => "✅",
            Status::Failure => "❌",
            Status::Info => "💡",
            Status::Question => "❓",
        }
    }
}
