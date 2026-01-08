mod cli;
mod memory;
mod cache;
use cli::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
