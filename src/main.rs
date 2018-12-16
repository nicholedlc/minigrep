use std::env;
use std::fs;
use std::process;

fn main() {
    // std::env::args returns an iterator of the command line args.
    // iterators produce a series of values, from which you can the `collect` method to turn it into a collection
    // collect is one function you often need to annotate (Rust isn’t able to infer the kind of collection you want)
    let args: Vec<String> = env::args().collect();

    // unwrap_or_else (defined on Result<T, E>) allows us to defined some custom, `non-panic!` error handling
    // if result is an `Ok` value it returns the inner value `Ok` is wrapping
    // if the value is an `Err` value, this method calls the anonymous fn defined and passed in as arg to `unwrap_or_else`
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1)
    });
    println!("Searching for {:?}", config.query);
    println!("In file {:?}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents)
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
