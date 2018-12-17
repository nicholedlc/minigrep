use std::error::Error;
use std::fs;

// `Box<dyn Error>` means the function will return a type that implements the Error trait
// but we don't have to specify what particular type the return value will be (`dyn` = dynamic)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
  // and the program will continue. If the value is an Err, the Err will be returned from the whole function
  // as if we had used the return keyword so the error value gets propagated to the calling code
  let contents = fs::read_to_string(config.filename)?;

  println!("With text:\n{}", contents);

  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  // `new` returns a Result with a Config instance in the success case
  // and a &'static str (the type of string literals) in the error case.
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
