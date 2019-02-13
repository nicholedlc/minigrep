use std::env;
use std::error::Error;
use std::fs;

// `Box<dyn Error>` means the function will return a type that implements the Error trait
// but we don't have to specify what particular type the return value will be (`dyn` = dynamic)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? --> If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
  // and the program will continue. If the value is an Err, the Err will be returned from the whole function
  // as if we had used the return keyword so the error value gets propagated to the calling code
  let contents = fs::read_to_string(config.filename)?;

  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }

  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  // Because we're taking ownership of `args` and we'll be mutating `args` by iterating over it,
  // we can add the `mut` keyword into the specification of the `args` parameter to make it mutable
  pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
    // `std::env::Args` implements the Iterator trait, so we can call the `next` method on it.
    // we do nothing with the first arg as it returns the name of the program
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    // If the `CASE_INSENSITIVE` env var is set to anything, `is_err` will return false and the program will perform a case-sensitive search.
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

// Lifetime parameters specify which argument lifetime is connected to the lifetime of the return value.
// In this case, we indicate that the returned vector should contain string slices that reference slices of the argument contents (rather than the argument query)
fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(query))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
