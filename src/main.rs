use std::env;
use std::process;

use minigrep;
use minigrep::Config;

fn main() {
    // std::env::args returns an iterator of the command line args.
    // iterators produce a series of values, from which you can the `collect` method to turn it into a collection
    // collect is one function you often need to annotate (Rust isn’t able to infer the kind of collection you want)
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else (defined on Result<T, E>) allows us to defined some custom, `non-panic!` error handling
    // if result is an `Ok` value it returns the inner value `Ok` is wrapping
    // if the value is an `Err` value, this method will pass the inner value of `Err` (the static string) to our closure
    // in the argument `err` of the anonymous function, which is defined and passed in as arg to `unwrap_or_else`
    let config = Config::new(&args).unwrap_or_else(|err| {
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
