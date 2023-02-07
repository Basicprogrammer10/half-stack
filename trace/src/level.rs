/// Log levels.
/// Used to control the verbosity of logging.
/// The default log level is [`Level::Error`].
/// 
/// | Level            | Description                       |
/// |------------------|-----------------------------------|
/// | [`Level::Off`]   | Disables all logging.             |
/// | [`Level::Error`] | For critical errors.              |
/// | [`Level::Info`]  | For general information.          |
/// | [`Level::Debug`] | For verbose debugging information.|
#[repr(u8)]
#[derive(Debug, Copy, Clone)]
#[rustfmt::skip]
pub enum Level {
    Off   = 0,
    Error = 1,
    Info  = 2,
    Debug = 3,
}

impl Level {
    /// Returns the log level as a string
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            Level::Off => "OFF",
            Level::Error => "ERROR",
            Level::Info => "INFO",
            Level::Debug => "DEBUG",
        }
    }

    /// Gets a color code for the log level.
    /// This is used to colorize the log output if color logging is enabled.
    pub(super) fn get_color(&self) -> &'static str {
        match self {
            Level::Off => unreachable!(),
            Level::Info => "\x1b[0m",
            Level::Error => "\x1b[31m",
            Level::Debug => "\x1b[36m",
        }
    }
}
