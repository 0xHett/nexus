#[allow(dead_code)]
mod console;
mod lexer;

use std::fs;
use std::io;
use std::path::Path;
use std::env;
use console::Console;
use lexer::*;

/// main() is the entry point for the compiler, it simply checks the environment's args for the
/// file(s) and compiler flags.
fn main() {
    let mut args = env::args();
    args.next();
    let arg_1 = args.next();

    if arg_1.is_none() { Console::prompt_error("Failed to parse args! Correct usage: nexus <file(s)> [options]."); }

    match args.collect::<Vec<_>>().len() {
        0 => {
            let _ = run(arg_1);
        },
        _ => {
            Console::prompt_error("Failed to parse args! Correct usage: nexus <file(s)> [options].");
        }
    }
}

/// run() checks the inputted file(s) extension and if it is ".nxs", it will read the file and pass
/// it to the lexer/tokenizer
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
            Console::prompt_error("Invalid file extension! Correct file extension: \".nxs\"!");    
            std::process::exit(1);
        },
        None => { 
            Console::prompt_error("Invalid file extension! Correct file extension: \".nxs\"!");
            std::process::exit(1);
        }
    }
    
    Console::prompt_info("Verbose - Reading inputted file...");

    let file_contents = fs::read_to_string(&filepath)?;
    // send that mfer
    Ok(())
}
