use std::env;
use std::fs;

fn main() {
    // std::env::args returns an iterator of the command line args.
    // iterators produce a series of values, from which you can the `collect` method to turn it into a collection
    // collect is one function you often need to annotate (Rust isn’t able to infer the kind of collection you want)
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);
    println!("Searching for {:?}", query);
    println!("In file {:?}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("With text:\n{}", contents)
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
