mod cli;
mod memory;

use cli::Shell;

fn main() {
    let mut shell = Shell::new();
    shell.run();
}
