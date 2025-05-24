mod config;
mod affirmations;
mod utils;
mod color;
mod mommy;

fn main() {
    let exit_code = mommy::mommy().map(|_| 0).unwrap_or_else(|e| { eprintln!("Error: {:?}", e); 1 });
    std::process::exit(exit_code);
}