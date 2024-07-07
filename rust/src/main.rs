use std::env;

fn main() {
  let arguments: Vec<String> = env::args().collect();

  let query = &arguments[1];

  println!("Query: {}", query);
}

