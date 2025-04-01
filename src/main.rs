#[allow(dead_code)]

mod console;

use std::io;
use console::ConsoleColor;

fn main() {
    println!("{0}", ConsoleColor::format_str("[Nexus] Compiler - Input your file >> ", ConsoleColor::Y));
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("test");
}
