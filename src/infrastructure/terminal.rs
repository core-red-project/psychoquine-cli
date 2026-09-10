use std::io::{self, IsTerminal};

pub struct Terminal;

impl Terminal {
    pub fn is_stdout_tty() -> bool {
        io::stdout().is_terminal()
    }

    pub fn is_stderr_tty() -> bool {
        io::stderr().is_terminal()
    }

    pub fn is_no_color_active() -> bool {
        std::env::var_os("NO_COLOR").is_some()
    }
}
