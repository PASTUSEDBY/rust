use std::{env, process};
use minigrep::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Too less args. Minimum 2 is required.");
        process::exit(1);
    }

    let ignore = env::var("IGNORE_CASE").is_ok();

    let config = Config::new(&args, ignore);

    if let Err(err) = run(config) {
        eprintln!("Something bad happened: {err}");
        process::exit(1);
    }
}
