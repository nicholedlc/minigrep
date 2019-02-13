use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // `env::args()` returns an iterator. Rather than collecting the iterator values
    //  into a vector and then passing a slice to Config::new, now we're passing ownership
    // of the iterator returned from env::args to Config::new directly
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1)
    });

    // Because run returns () in the success case, we only care about detecting an error,
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1)
    }
}
