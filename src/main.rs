mod cli;
mod memory;
mod cache;
mod vm;
use cli::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
