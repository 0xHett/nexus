//! This whole module is used to provide fancy text formatting and coloring, it also allows for
//! easy error, warning, and info prompting!

#[allow(dead_code)]
pub struct Console;

impl Console {
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";
    
    pub const R: &'static str = "\x1b[31m";
    pub const G: &'static str = "\x1b[32m";
    pub const B: &'static str = "\x1b[34m";
    pub const Y: &'static str = "\x1b[33m";
    pub const P: &'static str = "\x1b[35m";
    pub const W: &'static str = "\x1b[37m";
    pub const GR: &'static str = "\x1b[90m";
    
    /// colorizes a string
    pub fn format_str(string: &str, color: &str) -> String {
        format!("{0}{1}{2}", color, string, Console::RESET)
    }
    
    /// prompts an error to the console in a fancy way! *USE for providing an error/critcal to the
    /// user*
    pub fn prompt_error(message: &str) {
        let compiler_name: String = Self::format_str("Nexus", Console::BOLD);
        let compiler_div_l: String = Self::format_str("[", Console::GR);
        let compiler_div_r: String = Self::format_str("]", Console::GR);

        println!("{0}{1}{2} {3}", compiler_div_l, compiler_name, compiler_div_r, Console::format_str(&message, Console::R));
        std::process::exit(1);
    }

    /// prompts a warning to the console in a fancy way! *USE for providing a warning to the user*
    pub fn prompt_warning(message: &str) {
        let compiler_name: String = Self::format_str("Nexus", Console::BOLD);
        let compiler_div_l: String = Self::format_str("[", Console::GR);
        let compiler_div_r: String = Self::format_str("]", Console::GR);

        println!("{0}{1}{2} {3}", compiler_div_l, compiler_name, compiler_div_r, Console::format_str(&message, Console::Y));
    }

    /// prompts an info message to the console in a fancy way! *USE for verbose*
    pub fn prompt_info(message: &str) {
        let compiler_name: String = Self::format_str("Nexus", Console::BOLD);
        let compiler_div_l: String = Self::format_str("[", Console::GR);
        let compiler_div_r: String = Self::format_str("]", Console::GR);

        println!("{0}{1}{2} {3}", compiler_div_l, compiler_name, compiler_div_r, Console::format_str(&message, Console::GR));
    }
}
