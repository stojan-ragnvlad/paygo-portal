use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn create_sql_schema_from_csv(csv_table: &[u8]) -> String {
  return std::str::from_utf8(&csv_table[0..10]).unwrap().to_string();
}

