> This is still a work in progress, no I havent gotten to the lexer and abstract syntax tree implementations yet let alone the finalizer, so it's not usable yet.
## How to build
- You will need rust and gcc (for the linker, I plan to replace this later on with my own linker and assembler) installed, which you can install here [rust](https://www.rust-lang.org/tools/install) [gcc](https://gcc.gnu.org/install/binaries.html)
- git clone the project and run
```
cargo build --release
./target/release/nexus <input file> <compiler flags (optional)>
```
