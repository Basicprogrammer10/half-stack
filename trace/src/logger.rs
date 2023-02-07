use crate::Level;

pub struct Logger {
    level: u8,
    color: bool,
}

impl Default for Logger {
    fn default() -> Self {
        Self {
            level: Level::Debug as u8,
            color: true,
        }
    }
}

impl Logger {
    /// Creates a new [`Logger`] with level [`Level::Info`] and color enabled.
    pub fn new() -> Self {
        Default::default()
    }

    /// Sets the log level to `level`
    pub fn level(&mut self, level: Level) -> &mut Self {
        self.level = level as u8;
        self
    }

    //// En/disabled color in logging
    pub fn color(&mut self, color: bool) -> &mut Self {
        self.color = color;
        self
    }

    // Logs `msg` with `level` if that level or a lower one is enabled
    pub fn log(&self, level: Level, msg: &str) {
        if level as u8 > self.level {
            return;
        }

        println!(
            "[{}] {}{}{}",
            level.as_str(),
            if self.color { level.get_color() } else { "" },
            msg,
            if self.color { "\x1b[0m" } else { "" }
        );
    }

    /// Error log. ([`Level::Error`])
    pub fn error(&self, msg: impl AsRef<str>) {
        self.log(Level::Error, msg.as_ref());
    }

    /// Info log. ([`Level::Info`])
    pub fn info(&self, msg: impl AsRef<str>) {
        self.log(Level::Info, msg.as_ref());
    }

    /// Debug log. ([`Level::Debug`])
    pub fn debug(&self, msg: impl AsRef<str>) {
        self.log(Level::Debug, msg.as_ref());
    }
}
