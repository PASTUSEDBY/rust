use std::{env, process};
use minigrep::*;

fn main() {
    let args = env::args();
    let ignore = env::var("IGNORE_CASE").is_ok();

    let config = Config::build(args, ignore)
            .unwrap_or_else(|| {
                eprintln!("Too few arguments. Minimum 2 required");
                process::exit(0)
            });

    if let Err(err) = run(config) {
        eprintln!("Something bad happened: {err}");
        process::exit(1);
    }
}
