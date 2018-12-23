use std::env;
use std::error::Error;
use std::fs;

// `Box<dyn Error>` means the function will return a type that implements the Error trait
// but we don't have to specify what particular type the return value will be (`dyn` = dynamic)
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // If the value of the Result is an Ok, the value inside the Ok will get returned from this expression,
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

    // If the `CASE_INSENSITIVE` env var is set to anything, `is_err` will return false and the program will perform a case-sensitive search.
    let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    case_sensitive = match args.get(3) {
      None => case_sensitive,
      Some(_) => false,
    };

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
  let mut results = Vec::new();

  for line in contents.lines() {
    if line.contains(query) {
      results.push(line);
    }
  }

  results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let query = query.to_lowercase();
  let mut results = Vec::new();

  for line in contents.lines() {
    // if line.to_lowercase().contains(query.as_str()) {
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }

  results
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
