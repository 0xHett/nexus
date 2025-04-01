#[allow(dead_code)]
mod console;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;
use std::env;
use console::ConsoleColor;

fn main() {
    let mut args = env::args();
    args.next();
    let arg_1 = args.next();

    if arg_1.is_none() { error_prompt("Failed to parse args! Correct usage: nexus <file(s)> [options]."); }

    match args.collect::<Vec<_>>().len() {
        0 => {
            let _ = run(arg_1);
        },
        _ => {
            error_prompt("Failed to parse args! Correct usage: nexus <file(s)> [options].");
        }
    }
}

fn run(filepath: Option<String>) -> io::Result<()> {
    let filepath = match filepath {
        Some(path) => path,
        None => {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "File has no extension"));
        }
    };

    let path = Path::new(&filepath);
    match path.extension() {
        Some(_ext) if _ext == "nxs" => {},
        Some(_ext) => {
            error_prompt("Invalid file extension! Correct file extension: \".nxs\"!");    
            std::process::exit(1);
        },
        None => { 
            error_prompt("Invalid file extension! Correct file extension: \".nxs\"!");
            std::process::exit(1);
        }
    }

    let mut file = File::open(filepath)?;
    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    Ok(())
}

fn error_prompt(error_message: &str) {
    let compiler_name: String = ConsoleColor::format_str("Nexus", ConsoleColor::BOLD);
    let compiler_divl: String = ConsoleColor::format_str("[", ConsoleColor::GR);
    let compiler_divr: String = ConsoleColor::format_str("]", ConsoleColor::GR);
    
    println!("{0}{1}{2} {3}", &compiler_divl, &compiler_name, &compiler_divr, ConsoleColor::format_str(&error_message, ConsoleColor::R));
    std::process::exit(1);
}
