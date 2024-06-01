use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_random_string() -> String {
  let mut rng = thread_rng();
  let chars: String =
      (0..24).map(|_| rng.sample(Alphanumeric) as char).collect();

  return chars;
}

