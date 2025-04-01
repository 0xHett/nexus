#[allow(dead_code)]

pub struct ConsoleColor;

impl ConsoleColor {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    
    pub const R: &'static str = "\x1b[31m";
    pub const G: &'static str = "\x1b[32m";
    pub const B: &'static str = "\x1b[34m";
    pub const Y: &'static str = "\x1b[33m";
    pub const W: &'static str = "\x1b[37m";

    pub fn format_str(string: &str, color: &str) -> String {
        format!("{0}{1}{2}", color, string, ConsoleColor::RESET)
    }
}
