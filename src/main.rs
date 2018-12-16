use std::env;
use std::error::Error;
use std::fs;
use std::process;

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
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    // Because run returns () in the success case, we only care about detecting an error,
    // so we don’t need unwrap_or_else to return the unwrapped value because it would only be ()
    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1)
    }
}

// `Box<dyn Error>` means the function will return a type that implements the Error trait
// but we don't have to specify what particular type the return value will be (`dyn` = dynamic)
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
    // and the program will continue. If the value is an Err, the Err will be returned from the whole function
    // as if we had used the return keyword so the error value gets propagated to the calling code
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    // `new` returns a Result with a Config instance in the success case
    // and a &'static str (the type of string literals) in the error case.
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // `clone()` will make a full copy of the data for the Config instance to own,
        // which takes more time and memory than storing a reference to the string data.
        // However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references;
        // in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
