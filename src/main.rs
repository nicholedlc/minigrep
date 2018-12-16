use std::env;

fn main() {
    // std::env::args returns an iterator of the command line args.
    // iterators produce a series of values, from which you can the `collect` method to turn it into a collection
    // collect is one function you often need to annotate (Rust isn’t able to infer the kind of collection you want)
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {:?}", query);
    println!("In file {:?}", filename);
}