use std::env;
use std::fs;

fn main() {
    // std::env::args returns an iterator of the command line args.
    // iterators produce a series of values, from which you can the `collect` method to turn it into a collection
    // collect is one function you often need to annotate (Rust isn’t able to infer the kind of collection you want)
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
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
    fn new(args: &[String]) -> Config {
        // `clone()` will make a full copy of the data for the Config instance to own,
        // which takes more time and memory than storing a reference to the string data.
        // However, cloning the data also makes our code very straightforward because we don’t have to manage the lifetimes of the references;
        // in this circumstance, giving up a little performance to gain simplicity is a worthwhile trade-off.
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}
